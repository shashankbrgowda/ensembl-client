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

import { useAppDispatch, useAppSelector } from 'src/store';
import { selectedFiltersCount } from 'src/content/app/tools/biomart/state/biomartSelectors';
import { resetFilterData } from 'src/content/app/tools/biomart/state/biomartSlice';

import styles from '../BiomartForm.module.css';

const BiomartFiltersHeader = () => {
  const dispatch = useAppDispatch();
  const count = useAppSelector(selectedFiltersCount);

  const onReset = () => {
    dispatch(resetFilterData());
  };

  return (
    <div className={styles.columnsHeader}>
      <div className={styles.headerTitle}>
        <span>Filter</span>
        <span className={styles.counterClass}>{count}</span>
      </div>
      <div className={styles.headerSettings}>
        <span className={styles.reset} onClick={onReset}>
          Reset
        </span>
      </div>
    </div>
  );
};

export default BiomartFiltersHeader;
