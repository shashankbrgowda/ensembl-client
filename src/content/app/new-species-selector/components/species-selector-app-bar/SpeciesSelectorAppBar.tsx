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

import { useAppSelector } from 'src/store';

import * as urlFor from 'src/shared/helpers/urlHelper';

import { getCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';

import AppBar from 'src/shared/components/app-bar/AppBar';
import { HelpPopupButton } from 'src/shared/components/help-popup';
import SpeciesLozenge from 'src/shared/components/selected-species/SpeciesLozenge';
import SpeciesTabsWrapper from 'src/shared/components/species-tabs-wrapper/SpeciesTabsWrapper';
import GeneSearchButton from 'src/shared/components/gene-search-button/GeneSearchButton';
import GeneSearchCloseButton from 'src/shared/components/gene-search-button/GeneSearchCloseButton';

import type { CommittedItem } from 'src/content/app/species-selector/types/species-search';

import styles from './SpeciesSelectorAppBar.scss';

export const placeholderMessage =
  'Find and add your favourite species to use them across the site';

const PlaceholderMessage = () => (
  <div className={styles.placeholderMessage}>{placeholderMessage}</div>
);

type Props = {
  onGeneSearchToggle: () => void;
  isGeneSearchMode: boolean;
};

export const SpeciesSelectorAppBar = (props: Props) => {
  const selectedSpecies = useAppSelector(getCommittedSpecies);

  const mainContent =
    selectedSpecies.length > 0 ? (
      <SelectedSpeciesList selectedSpecies={selectedSpecies} {...props} />
    ) : (
      <PlaceholderMessage />
    );

  return (
    <AppBar
      appName="Species Selector"
      mainContent={mainContent}
      aside={<HelpPopupButton slug="species-selector-intro" />}
    />
  );
};

const SelectedSpeciesList = (
  props: Props & { selectedSpecies: CommittedItem[] }
) => {
  const navigate = useNavigate();

  const showSpeciesPage = (species: CommittedItem) => {
    const genomeIdForUrl = species.genome_tag ?? species.genome_id;
    const speciesPageUrl = urlFor.speciesPage({
      genomeId: genomeIdForUrl
    });

    navigate(speciesPageUrl);
  };

  const conditionalSpeciesProps = !props.isGeneSearchMode
    ? ({ onClick: showSpeciesPage, theme: 'blue' } as const)
    : ({ theme: 'grey' } as const);

  const selectedSpecies = props.selectedSpecies.map((species) => (
    <SpeciesLozenge
      key={species.genome_id}
      species={species}
      {...conditionalSpeciesProps}
    />
  ));

  const geneSearchButton = props.isGeneSearchMode ? (
    <GeneSearchCloseButton
      key="find-a-gene"
      onClick={props.onGeneSearchToggle}
    />
  ) : (
    <GeneSearchButton key="find-a-gene" onClick={props.onGeneSearchToggle} />
  );

  const speciesTabsWrapperContent = [...selectedSpecies, geneSearchButton];

  return <SpeciesTabsWrapper speciesTabs={speciesTabsWrapperContent} />;
};

export default SpeciesSelectorAppBar;
