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

import { useEffect, useRef, useCallback } from 'react';
import { RootState } from 'src/store';
import { useLocation, useNavigate } from 'react-router-dom';
import isEqual from 'lodash/isEqual';

import { useAppSelector, useAppDispatch } from 'src/store';
import { useUrlParams } from 'src/shared/hooks/useUrlParams';
import useGenomeBrowser from 'src/content/app/genome-browser/hooks/useGenomeBrowser';

import * as urlFor from 'src/shared/helpers/urlHelper';
import {
  getChrLocationFromStr,
  getChrLocationStr
} from '../helpers/browserHelper';
import {
  buildFocusIdForUrl,
  parseFocusIdFromUrl,
  parseFocusObjectId,
  buildFocusObjectId
} from 'src/shared/helpers/focusObjectHelpers';

import { fetchGenomeData } from 'src/shared/state/genome/genomeSlice';
import {
  setActiveGenomeId,
  setDataFromUrlAndSave
} from 'src/content/app/genome-browser/state/browser-general/browserGeneralSlice';
import { fetchFocusObject } from 'src/content/app/genome-browser/state/focus-object/focusObjectSlice';

import { getEnabledCommittedSpecies } from 'src/content/app/species-selector/state/speciesSelectorSelectors';
import {
  getBrowserActiveGenomeId,
  getBrowserActiveFocusObjectIds
} from '../state/browser-general/browserGeneralSelectors';
import { getFocusObjectById } from 'src/content/app/genome-browser/state/focus-object/focusObjectSelectors';
import { getAllChrLocations } from '../state/browser-general/browserGeneralSelectors';

/*
 * Possible urls that the GenomeBrowser page has to deal with:
 * - /browser (no genome id selected)
 * - /browser/:genome_id (no focus object or location selected)
 * - /browser/:genome_id?focus=:focus_object_id (no location selected)
 * - /browser/:genome_id?focus=:focus_object_id&location=:location_id
 *
 * We are not dealing with urls where genome id and location id are defined, but focus id isn't
 *
 *  NOTE: focus id in the url has the format <type>:<id>;
 *  but internally in the app we will use the focus id format that is <genome_id>:<type>:<id>
 *  because without the genome id the focus id is not unique enough to be used as key in our Redux store
 */

const useBrowserRouting = () => {
  const firstRenderRef = useRef(true);
  const params = useUrlParams<'genomeId'>('/genome-browser/:genomeId');
  const { search } = useLocation(); // from document.location provided by the router
  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  const { genomeId } = params;
  const urlSearchParams = new URLSearchParams(search);
  const focus = urlSearchParams.get('focus') || null;
  const location = urlSearchParams.get('location') || null;

  const activeGenomeId = useAppSelector(getBrowserActiveGenomeId);
  const committedSpecies = useAppSelector(getEnabledCommittedSpecies);
  const allChrLocations = useAppSelector(getAllChrLocations);
  const allActiveFocusObjectIds = useAppSelector(
    getBrowserActiveFocusObjectIds
  );

  const activeFocusObjectId = genomeId
    ? allActiveFocusObjectIds[genomeId]
    : null;
  const focusObject = useAppSelector((state: RootState) =>
    getFocusObjectById(state, activeFocusObjectId || '')
  );

  const newFocusId = focus
    ? buildNewFocusObjectId(genomeId as string, focus)
    : null;
  const chrLocation = location ? getChrLocationFromStr(location) : null;
  const { genomeBrowser, changeFocusObject, changeBrowserLocation } =
    useGenomeBrowser();

  useEffect(() => {
    if (!genomeId) {
      // handling navigation to /browser
      // select either the species that the user viewed during the previous visit,
      // or the first selected species
      const selectedSpecies = committedSpecies.find(
        ({ genome_id }) => genome_id === activeGenomeId
      );
      const firstCommittedSpecies = committedSpecies[0];
      if (selectedSpecies) {
        changeGenomeId(selectedSpecies.genome_id);
      } else if (firstCommittedSpecies) {
        changeGenomeId(firstCommittedSpecies.genome_id);
      }
      return;
    }

    const payload = {
      activeGenomeId: genomeId,
      activeFocusObjectId: newFocusId,
      chrLocation
    };

    if (!activeGenomeId) {
      /*
        Means that this is the first time user visits genome browser.
        We need to make sure that active genome id is set properly in redux
        for the actions below (e.g. changeFocusObject) to work
      */
      dispatch(setActiveGenomeId(genomeId));
    }

    if (!focus && activeFocusObjectId) {
      const newFocus = buildFocusIdForUrl(
        parseFocusObjectId(activeFocusObjectId)
      );
      navigate(urlFor.browser({ genomeId, focus: newFocus }), {
        replace: true
      });
    } else if (newFocusId && !chrLocation) {
      /*
       changeFocusObject needs to be called before setDataFromUrlAndSave
       because it will also try to bookmark the Ensembl object that is stored in redux state
       before it gets changed by setDataFromUrlAndSave
      */
      changeFocusObject(newFocusId);
    } else if (focus && chrLocation) {
      const isSameLocationAsInRedux =
        activeGenomeId && isEqual(chrLocation, allChrLocations[activeGenomeId]);
      const isFirstRender = firstRenderRef.current;
      if (genomeBrowser) {
        if (!isSameLocationAsInRedux || isFirstRender) {
          const { objectId } = parseFocusIdFromUrl(focus);

          changeBrowserLocation({
            genomeId,
            focusId: objectId,
            chrLocation
          });

          firstRenderRef.current = false;
        }
      }
    }
    dispatch(setDataFromUrlAndSave(payload));
  }, [genomeId, focus, genomeBrowser, location]);

  useEffect(() => {
    if (genomeId) {
      dispatch(fetchGenomeData(genomeId));
    }
  }, [genomeId]);

  useEffect(() => {
    if (!focusObject && activeFocusObjectId) {
      dispatch(fetchFocusObject(activeFocusObjectId));
    }
  }, [focusObject, activeFocusObjectId]);

  const changeGenomeId = useCallback(
    (genomeId: string) => {
      const chrLocation = allChrLocations[genomeId];
      const activeFocusObjectId = allActiveFocusObjectIds[genomeId];
      const focusIdForUrl = activeFocusObjectId
        ? buildFocusIdForUrl(activeFocusObjectId)
        : null;

      const nextUrlParams = {
        genomeId,
        focus: focusIdForUrl,
        location: chrLocation ? getChrLocationStr(chrLocation) : null
      };

      dispatch(setActiveGenomeId(genomeId));
      // Consider it as first render when we change genome
      firstRenderRef.current = true;

      // In case the url is simply /genome-browser, use the `replace` history method to redirect the user further.
      // This will allow the user to return from the next page (genome browser with genome id selected)
      // back to the page from which they have navigated to the current one. If the `push` method is used,
      // the user will not be able to get back past this page, because the url without genome id will remain
      // in the history, and the user will be forcefully redirected to the url with the genome id.
      //
      // NOTE: the logic will likely change in the future when /genome-browser without the selected genome id
      // becomes a valid searchable page in its own right.

      navigate(urlFor.browser(nextUrlParams), { replace: !params.genomeId });
    },
    [params.genomeId]
  );

  return {
    changeGenomeId
  };
};

const buildNewFocusObjectId = (genomeId: string, focusFromUrl: string) => {
  const parsedFocus = parseFocusIdFromUrl(focusFromUrl);
  return buildFocusObjectId({ genomeId, ...parsedFocus });
};

export default useBrowserRouting;
