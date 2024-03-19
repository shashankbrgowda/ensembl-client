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

import React from 'react';
import { useNavigate } from 'react-router-dom';

import useSpeciesSelectorAnalytics from 'src/content/app/species-selector/hooks/useSpeciesSelectorAnalytics';

import * as urlFor from 'src/shared/helpers/urlHelper';

import SpeciesSearchField from 'src/content/app/species-selector/components/species-search-field/SpeciesSearchField';
import PopularSpeciesList from 'src/content/app/species-selector/components/popular-species-list/PopularSpeciesList';

import styles from './SpeciesSelectorMainView.module.css';

const SpeciesSelectorMainView = () => {
  const navigate = useNavigate();
  const { trackSpeciesSearchQuery } = useSpeciesSelectorAnalytics();

  const onSearchSubmit = (query: string) => {
    trackSpeciesSearchQuery(query);
    navigate(
      urlFor.speciesSelectorSearch({
        query
      })
    );
  };

  return (
    <div className={styles.main}>
      <div className={styles.searchPanel}>
        <SpeciesSearchField onSearchSubmit={onSearchSubmit} />
      </div>
      <div className={styles.popularSpecies}>
        <PopularSpeciesList />
      </div>
    </div>
  );
};

export default SpeciesSelectorMainView;
