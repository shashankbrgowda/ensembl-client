/**
 * See the NOTICE file distributed with this work for additional information
 * regarding copyright ownership.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import React, { useState } from 'react';
import { useSelector } from 'react-redux';
import upperFirst from 'lodash/upperFirst';

import { useAppDispatch } from 'src/store';

import {
  search,
  clearSearch,
  updateQuery
} from 'src/shared/state/in-app-search/inAppSearchSlice';
import {
  getSearchQuery,
  getSearchResults
} from 'src/shared/state/in-app-search/inAppSearchSelectors';

import { pluralise } from 'src/shared/helpers/formatters/pluralisationFormatter';
import { getCommaSeparatedNumber } from 'src/shared/helpers/formatters/numberFormatter';

import analyticsTracking from 'src/services/analytics-service';

import SearchField from 'src/shared/components/search-field/SearchField';
import { PrimaryButton } from 'src/shared/components/button/Button';
import QuestionButton, {
  QuestionButtonOption
} from 'src/shared/components/question-button/QuestionButton';
import CloseButton from 'src/shared/components/close-button/CloseButton';
import { CircleLoader } from 'src/shared/components/loader';
import InAppSearchMatches from './InAppSearchMatches';

import type { RootState } from 'src/store';
import type { AppName } from 'src/shared/state/in-app-search/inAppSearchSlice';

import styles from './InAppSearch.scss';

export type InAppSearchMode = 'interstitial' | 'sidebar';

export type Props = {
  app: AppName;
  genomeId: string;
  genomeIdForUrl: string; // this should be a temporary measure; it should be returned by search api
  mode: InAppSearchMode;
  onSearchSubmit?: (query: string) => void;
};

const InAppSearch = (props: Props) => {
  const { app, genomeId, genomeIdForUrl, mode } = props;
  const [isLoading, setIsLoading] = useState(false);
  const query = useSelector((state: RootState) =>
    getSearchQuery(state, app, genomeId)
  );
  const searchResult = useSelector((state: RootState) =>
    getSearchResults(state, app, genomeId)
  );
  const dispatch = useAppDispatch();

  const onQueryChange = (query: string) => {
    dispatch(updateQuery({ app, genomeId, query }));
  };

  const onSearchSubmit = async () => {
    setIsLoading(true);
    props.onSearchSubmit?.(query);

    const searchParams = {
      app,
      genome_id: genomeId,
      query,
      page: 1,
      per_page: 50
    };

    try {
      await dispatch(search(searchParams));

      if (app === 'entityViewer') {
        analyticsTracking.trackEvent({
          category: `${app}_${mode}_search`,
          action: 'submit_search',
          label: query
        });
      }
    } finally {
      setIsLoading(false);
    }
  };

  const clear = () => {
    dispatch(clearSearch({ app, genomeId }));
  };

  const questionButtonHint =
    'Find a gene using a stable ID (versioned or un-versioned), symbol or synonym; wildcards are also supported';

  const rightCorner = query ? (
    <CloseButton onClick={clear} />
  ) : (
    <QuestionButton
      helpText={questionButtonHint}
      styleOption={QuestionButtonOption.INPUT}
    />
  );

  return (
    <div>
      <div
        className={getInAppSearchTopStyles(mode)}
        data-test-id="in-app search top"
      >
        <div className={styles.label}>Find a gene in this species</div>
        <div className={styles.searchFieldWrapper}>
          <SearchField
            placeholder="Gene ID or name..."
            search={query}
            onChange={onQueryChange}
            onSubmit={onSearchSubmit}
            className={styles.searchField}
            rightCorner={rightCorner}
            size={mode === 'interstitial' ? 'large' : 'small'}
          />
        </div>
        <PrimaryButton
          onClick={onSearchSubmit}
          className={styles.searchButton}
          isDisabled={!query || isLoading}
        >
          Go
        </PrimaryButton>
        {!isLoading && searchResult && (
          <div className={styles.hitsCount}>
            <span className={styles.hitsNumber}>
              {getCommaSeparatedNumber(searchResult.meta.total_hits)}
            </span>{' '}
            {pluralise('gene', searchResult.meta.total_hits)}
          </div>
        )}
      </div>
      {isLoading ? (
        <CircleLoader className={styles.spinner} size="small" />
      ) : (
        searchResult && (
          <InAppSearchMatches
            {...searchResult}
            app={app}
            mode={mode}
            genomeIdForUrl={genomeIdForUrl}
          />
        )
      )}
    </div>
  );
};

const getInAppSearchTopStyles = (mode: InAppSearchMode) => {
  return styles[`inAppSearchTop${upperFirst(mode)}`];
};

export default InAppSearch;
