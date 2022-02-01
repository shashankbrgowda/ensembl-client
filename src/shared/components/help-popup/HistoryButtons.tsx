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
import classNames from 'classnames';

import { ReactComponent as BackIcon } from 'static/img/browser/navigate-left.svg';
import { ReactComponent as ForwardIcon } from 'static/img/browser/navigate-right.svg';

import styles from '../help-popup/HelpPopupBody.scss';

type HistoryButtonsProps = {
  onHistoryBack: () => void;
  onHistoryForward: () => void;
  hasPrevious: boolean;
  hasNext: boolean;
};

const HistoryButtons = (props: HistoryButtonsProps) => {
  const historyForwardClasses = classNames(
    styles.historyButton,
    props.hasNext ? styles.historyButtonActive : styles.historyButtonInactive
  );
  const historyBackClasses = classNames(
    styles.historyButton,
    props.hasPrevious
      ? styles.historyButtonActive
      : styles.historyButtonInactive
  );

  return (
    <div className={styles.historyButtons}>
      <BackIcon className={historyBackClasses} onClick={props.onHistoryBack} />
      <ForwardIcon
        className={historyForwardClasses}
        onClick={props.onHistoryForward}
      />
    </div>
  );
};

export default HistoryButtons;