import { createAction } from 'typesafe-actions';
import { ActionCreator, Action } from 'redux';
import { batch } from 'react-redux';
import { push, replace } from 'connected-react-router';
import { ThunkAction } from 'redux-thunk';

import * as urlHelper from 'src/shared/helpers/urlHelper';

import { getCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';
import {
  getEntityViewerActiveGenomeId,
  getEntityViewerActiveEnsObjectIds,
  getEntityViewerActiveEnsObject,
  getEntityViewerActiveEnsObjectId
} from './entityViewerGeneralSelectors';
import { getGenomeInfoById } from 'src/shared/state/genome/genomeSelectors';

import { fetchGenomeData } from 'src/shared/state/genome/genomeActions';

import { EntityViewerParams } from 'src/content/app/entity-viewer/EntityViewer';
import { RootState } from 'src/store';
import { fetchEnsObject } from 'src/shared/state/ens-object/ensObjectActions';

export const setActiveGenomeId = createAction(
  'entity-viewer/set-active-genome-id'
)<string>();

export const setDataFromUrl: ActionCreator<ThunkAction<
  void,
  any,
  null,
  Action<string>
>> = (params: EntityViewerParams) => (dispatch, getState: () => RootState) => {
  const state = getState();
  const activeGenomeId = getEntityViewerActiveGenomeId(state);
  const activeEntityId = getEntityViewerActiveEnsObjectId(state);
  if (!params.genomeId) {
    dispatch(setDefaultActiveGenomeId());
  } else if (params.genomeId !== activeGenomeId) {
    dispatch(setActiveGenomeId(params.genomeId));
    dispatch(fetchGenomeData(params.genomeId));
    // TODO: when backend is ready, entity info may also need fetching
  } else {
    // TODO: when backend is ready, fetch entity info
    const genomeInfo = getGenomeInfoById(state, activeGenomeId);
    if (!genomeInfo) {
      dispatch(fetchGenomeData(activeGenomeId));
    }
  }

  if (params.entityId && params.entityId !== activeEntityId) {
    dispatch(updateEnsObject(params.entityId));
  }
};

export const setDefaultActiveGenomeId: ActionCreator<ThunkAction<
  void,
  any,
  null,
  Action<string>
>> = () => (dispatch, getState: () => RootState) => {
  const state = getState();
  const [firstCommittedSpecies] = getCommittedSpecies(state);
  const activeGenomeId = firstCommittedSpecies.genome_id;
  const newUrl = urlHelper.entityViewer({ genomeId: activeGenomeId });
  batch(() => {
    dispatch(setActiveGenomeId(activeGenomeId));
    dispatch(replace(newUrl));
  });
};

export const changeActiveGenomeId: ActionCreator<ThunkAction<
  void,
  any,
  null,
  Action<string>
>> = (genomeId: string) => (dispatch) => {
  const newUrl = urlHelper.entityViewer({ genomeId });
  batch(() => {
    dispatch(setActiveGenomeId(genomeId));
    dispatch(push(newUrl));
  });
};

export const updateEntityViewerActiveEnsObjectIds = createAction(
  'entity-viewer/update-active-ens-object-ids'
)<{ [objectId: string]: string }>();

export const updateEnsObject: ActionCreator<ThunkAction<
  void,
  any,
  null,
  Action<string>
>> = (activeEnsObjectId: string) => {
  return (dispatch, getState: () => RootState) => {
    const state = getState();
    const activeGenomeId = getEntityViewerActiveGenomeId(state);
    if (!activeGenomeId) {
      return;
    }
    const currentActiveEnsObjectIds = getEntityViewerActiveEnsObjectIds(state);
    const updatedActiveEnsObjectIds = {
      ...currentActiveEnsObjectIds,
      [activeGenomeId]: activeEnsObjectId
    };

    const currentEnsObject = getEntityViewerActiveEnsObject(state);

    dispatch(updateEntityViewerActiveEnsObjectIds(updatedActiveEnsObjectIds));
    if (!currentEnsObject) {
      dispatch(fetchEnsObject(activeEnsObjectId));
    }
  };
};