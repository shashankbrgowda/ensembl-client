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

import queryString from 'query-string';

import config from 'config';

export const home = () => '/';
export const speciesSelector = () => '/species-selector';
export const customDownload = () => '/custom-download';

type BrowserUrlParams = {
  genomeId?: string | null;
  focus?: string | null;
  location?: string | null;
};

type EntityViewerUrlParams = {
  genomeId?: string | null;
  entityId?: string | null;
  view?: string | null;
  proteinId?: string | null;
};

type SpeciesPageUrlParams = {
  genomeId: string;
};

export const speciesPage = (params: SpeciesPageUrlParams) => {
  const speciesPageRootPath = '/species';

  return `${speciesPageRootPath}/${params.genomeId}`;
};

export const browser = (params?: BrowserUrlParams) => {
  const browserRootPath = '/genome-browser';
  if (params) {
    const path = `${browserRootPath}/${params.genomeId}`;
    // NOTE: if a parameter passed to queryString is null, it will still get into query;
    // so assign it to undefined in order to omit it from the query
    const query = queryString.stringify(
      {
        focus: params.focus || undefined,
        location: params.location || undefined
      },
      {
        encode: false
      }
    );
    return query ? `${path}?${query}` : path;
  } else {
    return browserRootPath;
  }
};

export const entityViewer = (params?: EntityViewerUrlParams) => {
  if (!params?.genomeId && params?.entityId) {
    // this should never happen; this combination doesn't make sense
    throw 'Malformed Entity Viewer url';
  }

  const genomeId = params?.genomeId || '';
  const entityId = params?.entityId || '';
  let path = '/entity-viewer';
  if (genomeId) {
    path += `/${genomeId}`;
  }
  if (entityId) {
    path += `/${entityId}`;
  }
  const query = queryString.stringify(
    {
      view: params?.view || undefined,
      protein_id: params?.proteinId || undefined
    },
    {
      encode: false
    }
  );

  return query ? `${path}?${query}` : path;
};

type RefgetUrlParams = {
  checksum: string;
  start?: number;
  end?: number;
};

/**
 * Refget is using a zero-based coordinate space,
 * with start coordinate inclusive and end exclusive.
 * To translate from Ensembl coordinate space to Refget's,
 * we subtract 1 from the start coordinate
 * and leave the end coordinate as is.
 */
export const refget = (params: RefgetUrlParams) => {
  const { checksum, start, end } = params;
  const searchParams = new URLSearchParams();
  if (typeof start === 'number') {
    const refgetStart = start - 1;
    searchParams.append('start', `${refgetStart}`);
  }
  if (typeof end === 'number') {
    searchParams.append('end', `${end}`);
  }
  searchParams.append('accept', 'text/plain');

  return `${
    config.refgetBaseUrl
  }/sequence/${checksum}?${searchParams.toString()}`;
};