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

import { useState } from 'react';

import useRefWithRerender from 'src/shared/hooks/useRefWithRerender';

import TextButton from 'src/shared/components/text-button/TextButton';
import { Toolbox, ToolboxPosition } from 'src/shared/components/toolbox';

import styles from './VepResultsAllele.module.css';

type Props = {
  sequence: string;
};

const MAX_DISPLAY_LENGTH = 5;

const AlleleSequence = ({ sequence }: Props) => {
  const [anchorRef, setAnchorRef] = useRefWithRerender<HTMLElement>(null);
  const [shouldShowTooltip, setShouldShowTooltip] = useState(false);

  if (sequence.length <= MAX_DISPLAY_LENGTH) {
    return sequence;
  }

  const onClick = () => {
    setShouldShowTooltip(!shouldShowTooltip);
  };

  const onOutsideClick = () => {
    setShouldShowTooltip(false);
  };

  const displaySequence = sequence.slice(0, MAX_DISPLAY_LENGTH) + '…';

  return (
    <div>
      <TextButton
        ref={setAnchorRef}
        onClick={onClick}
        style={{ position: 'relative' }}
      >
        {displaySequence}
      </TextButton>
      <div className={styles.sequenceLength}>{sequence.length}</div>
      {anchorRef.current && shouldShowTooltip && (
        <Toolbox
          onOutsideClick={onOutsideClick}
          anchor={anchorRef.current}
          position={ToolboxPosition.RIGHT}
        >
          <div className={styles.toolboxContents}>
            <div className={styles.sequence}>{sequence}</div>
          </div>
        </Toolbox>
      )}
    </div>
  );
};

export default AlleleSequence;