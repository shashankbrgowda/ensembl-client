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

import { combineReducers } from '@reduxjs/toolkit';
import drawer from 'src/content/app/genome-browser/state/drawer/drawerSlice';
import browserGeneral from 'src/content/app/genome-browser/state/browser-general/browserGeneralSlice';
import browserNav from 'src/content/app/genome-browser/state/browser-nav/browserNavSlice';
import trackConfig from 'src/content/app/genome-browser/state/track-config/trackConfigSlice';
import trackPanel from 'src/content/app/genome-browser/state/track-panel/trackPanelSlice';
import focusObjects from 'src/content/app/genome-browser/state/focus-object/focusObjectSlice';

export default combineReducers({
  drawer,
  browserGeneral,
  browserNav,
  trackConfig,
  trackPanel,
  focusObjects
});