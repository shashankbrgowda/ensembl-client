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

import React, { StrictMode } from 'react';
import { hydrate } from 'react-dom';
import { Provider } from 'react-redux';
import { CookiesProvider } from 'react-cookie';
import { HelmetProvider } from 'react-helmet-async';
import { BrowserRouter } from 'react-router-dom';

import ensureBrowserSupport from 'src/shared/helpers/browserSupport';

import { Provider as IndexedDBProvider } from 'src/shared/contexts/IndexedDBContext';
import configureStore from './store';
import Root from './root/Root';

import { registerSW } from './registerServiceWorker';

import './styles/main';

ensureBrowserSupport();

const store = configureStore();

hydrate(
  <StrictMode>
    <CookiesProvider>
      <IndexedDBProvider>
        <Provider store={store}>
          <BrowserRouter>
            <HelmetProvider>
              <Root />
            </HelmetProvider>
          </BrowserRouter>
        </Provider>
      </IndexedDBProvider>
    </CookiesProvider>
  </StrictMode>,
  document.getElementById('ens-app')
);

if (module.hot) {
  module.hot.accept();
} else {
  registerSW();
}