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

import {
  parseFocusObjectId,
  buildFocusObjectId,
  parseFocusIdFromUrl,
  buildFocusIdForUrl
} from 'src/shared/helpers/focusObjectHelpers';

import { useAppSelector } from 'src/store';
import usePrevious from 'src/shared/hooks/usePrevious';
import { useUrlParams } from 'src/shared/hooks/useUrlParams';
import { useGenomeInfoQuery } from 'src/shared/state/genome/genomeApiSlice';

import {
  getEntityViewerActiveGenomeId,
  getEntityViewerActiveEntityId,
  getEntityViewerActiveEntityIds
} from 'src/content/app/entity-viewer/state/general/entityViewerGeneralSelectors';
import { getCommittedSpeciesById } from 'src/content/app/species-selector/state/speciesSelectorSelectors';

import {
  EntityViewerIdsContext,
  type EntityViewerIdsContextType
} from './EntityViewerIdsContext';

import type { FetchBaseQueryError } from '@reduxjs/toolkit/query';

const EntityViewerIdsContextProvider = (props: {
  children: React.ReactNode;
}) => {
  const activeGenomeId = useAppSelector(getEntityViewerActiveGenomeId);
  const allActiveEntityIds = useAppSelector(getEntityViewerActiveEntityIds);
  const activeEntityId = useAppSelector(getEntityViewerActiveEntityId);
  const savedGenomeInfo = useAppSelector((state) =>
    getCommittedSpeciesById(state, activeGenomeId ?? '')
  );
  const previousActiveGenomeId = usePrevious(activeGenomeId);
  const previousActiveEntityId = usePrevious(activeEntityId);
  const params = useUrlParams<'genomeId' | 'entityId'>([
    '/entity-viewer/:genomeId',
    '/entity-viewer/:genomeId/:entityId'
  ]);
  const { genomeId: genomeIdInUrl, entityId: entityIdInUrl } = params;

  const { currentData: genomeInfo, error } = useGenomeInfoQuery(
    genomeIdInUrl ?? '',
    {
      skip: !genomeIdInUrl
    }
  );

  const genomeId = genomeInfo?.genomeId;
  const genomeIdForUrl =
    genomeIdInUrl ??
    genomeInfo?.genomeTag ??
    genomeInfo?.genomeId ??
    savedGenomeInfo?.genome_tag ??
    savedGenomeInfo?.genome_id;

  let entityId;
  let parsedEntityId;
  let isMalformedEntityId = false;

  if (genomeId && params.entityId) {
    try {
      parsedEntityId = {
        genomeId,
        ...parseFocusIdFromUrl(params.entityId)
      };
      entityId = buildFocusObjectId(parsedEntityId);
    } catch {
      isMalformedEntityId = true;
    }
  }

  // FIXME: sometimes we actually want to generate entity id for url from a provided genome id
  // (e.g. from active entity id); whereas other times, we want to use the entity id from the url
  const entityIdForUrl =
    genomeId && allActiveEntityIds[genomeId]
      ? buildFocusIdForUrl(parseFocusObjectId(allActiveEntityIds[genomeId]))
      : undefined;

  const hasActiveGenomeIdChanged = Boolean(
    activeGenomeId &&
      previousActiveGenomeId &&
      activeGenomeId !== previousActiveGenomeId
  );

  const hasActiveEntityIdChanged = Boolean(
    activeEntityId &&
      previousActiveEntityId &&
      activeEntityId !== previousActiveEntityId
  );

  const isMissingGenomeId =
    typeof (error as FetchBaseQueryError)?.status === 'number' &&
    (error as FetchBaseQueryError).status >= 400; // FIXME change status to 404 when the backend behaves

  const context: EntityViewerIdsContextType = {
    activeGenomeId,
    activeEntityId,
    genomeIdInUrl,
    entityIdInUrl,
    genomeIdForUrl,
    entityIdForUrl,
    genomeId,
    entityId,
    parsedEntityId,
    hasActiveGenomeIdChanged,
    hasActiveEntityIdChanged,
    isMissingGenomeId,
    isMalformedEntityId
  };

  return (
    <EntityViewerIdsContext.Provider value={context}>
      {props.children}
    </EntityViewerIdsContext.Provider>
  );
};

export default EntityViewerIdsContextProvider;