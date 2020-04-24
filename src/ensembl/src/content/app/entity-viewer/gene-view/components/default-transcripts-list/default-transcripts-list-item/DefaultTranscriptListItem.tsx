import React, { useState } from 'react';

import { getFeatureCoordinates } from 'src/content/app/entity-viewer/shared/helpers/entity-helpers';

import UnsplicedTranscript from 'src/content/app/entity-viewer/gene-view/components/unspliced-transcript/UnsplicedTranscript';
import TranscriptsListItemInfo from '../transcripts-list-item-info/TranscriptsListItemInfo';

import { Gene } from 'src/content/app/entity-viewer/types/gene';
import { Transcript } from 'src/content/app/entity-viewer/types/transcript';
import { TicksAndScale } from 'src/content/app/entity-viewer/gene-view/components/base-pairs-ruler/BasePairsRuler';

import transcriptsListStyles from '../DefaultTranscriptsList.scss';
import styles from './DefaultTranscriptListItem.scss';

type Props = {
  gene: Gene;
  transcript: Transcript;
  rulerTicks: TicksAndScale;
};

// NOTE: the width of the middle column is the same as the width of GeneOverviewImage, i.e. 695px

const DefaultTranscriptListItem = (props: Props) => {
  const { scale } = props.rulerTicks;
  const { start: geneStart } = getFeatureCoordinates(props.gene);
  const { start: transcriptStart, end: transcriptEnd } = getFeatureCoordinates(
    props.transcript
  );
  const transcriptStartX = scale(transcriptStart - geneStart); // FIXME In future, this should be done using relative position of transcript in gene
  const transcriptWidth = scale(transcriptEnd - transcriptStart); // FIXME  this too should be based on relative coordinates of transcript
  const style = {
    transform: `translateX(${transcriptStartX}px)`,
    cursor: 'pointer'
  };

  const [shouldShowInfo, setShouldShowInfo] = useState(false);
  const toggleListItemInfo = () => setShouldShowInfo(!shouldShowInfo);

  return (
    <div className={styles.defaultTranscriptListItem}>
      <div className={transcriptsListStyles.row}>
        <div className={transcriptsListStyles.left}>Left</div>
        <div
          className={transcriptsListStyles.middle}
          onClick={toggleListItemInfo}
        >
          <div style={style}>
            <UnsplicedTranscript
              transcript={props.transcript}
              width={transcriptWidth}
              standalone={true}
            />
          </div>
        </div>
        <div
          className={transcriptsListStyles.right}
          onClick={toggleListItemInfo}
        >
          <span className={styles.transcriptId}>{props.transcript.id}</span>
        </div>
      </div>
      {shouldShowInfo ? (
        <TranscriptsListItemInfo
          gene={props.gene}
          transcript={props.transcript}
        />
      ) : null}
    </div>
  );
};

export default DefaultTranscriptListItem;