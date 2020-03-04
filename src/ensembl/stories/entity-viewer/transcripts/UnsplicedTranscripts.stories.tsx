import React, { useState, useEffect, useRef } from 'react';
import { scaleLinear } from 'd3';
import classNames from 'classnames';
import { storiesOf } from '@storybook/react';

import { getTranscriptData } from './transcriptData';
import { getFeatureCoordinates } from 'src/content/app/entity-viewer/shared/helpers/entity-helpers';

import UnsplicedTranscript from 'src/content/app/entity-viewer/gene-view/components/unspliced-transcript/UnsplicedTranscript';

import { Gene as GeneType } from 'src/content/app/entity-viewer/types/gene';
import { Transcript as TranscriptType } from 'src/content/app/entity-viewer/types/transcript';

import styles from './UnsplicedTranscripts.stories.scss';

const GRAPHIC_WIDTH = 1200;
const VERTICAL_SPACE = 50;
const BRCA2Id = 'ENSG00000139618';
// single transcript id example: 'ENST00000544455';

const UnsplicedTranscriptsStory = () => {
  const [id, setId] = useState(BRCA2Id);
  const [data, setData] = useState<GeneType | TranscriptType | null>(null);

  useEffect(() => {
    getTranscriptData(id).then(setData);
  }, [id]);

  const onIdChange = (id: string) => {
    setId(id);
  };

  let content = null;

  if (data?.type === 'Transcript') {
    content = (
      <UnsplicedTranscript
        transcript={data}
        width={GRAPHIC_WIDTH}
        classNames={{
          transcript: styles.transcript,
          exon: styles.exon
        }}
        standalone={true}
      />
    );
  } else if (data?.type === 'Gene') {
    content = <MultipleTranscripts gene={data} />;
  }

  return (
    <div className={styles.container}>
      <FeatureIdForm id={id} onChange={onIdChange} />
      {content}
    </div>
  );
};

const MultipleTranscripts = (props: { gene: GeneType }) => {
  const [view, setView] = useState('expanded');
  const { gene } = props;
  const { start: geneStart, end: geneEnd } = getFeatureCoordinates(props.gene);

  const changeView = (view: string) => setView(view);

  const viewToggle = (
    <div className={styles.viewToggle}>
      <span onClick={() => changeView('expanded')}>Extended view</span>
      <span onClick={() => changeView('collapsed')}>Collapsed view</span>
    </div>
  );

  // won't work on circular chromosomes
  const scale = scaleLinear()
    .domain([geneStart, geneEnd])
    .range([0, GRAPHIC_WIDTH]);

  const renderedTranscripts = gene.transcripts.map((transcript, index) => {
    const {
      start: transcriptStart,
      end: transcriptEnd
    } = getFeatureCoordinates(transcript);
    const startX = scale(transcriptStart);
    const endX = scale(transcriptEnd);
    const y = view === 'expanded' ? index * VERTICAL_SPACE : 10;
    const width = Math.floor(endX - startX);
    return (
      <g key={index} transform={`translate(${startX} ${y})`}>
        <UnsplicedTranscript
          transcript={transcript}
          width={width}
          classNames={{
            transcript: styles.transcript,
            exon: styles.exon
          }}
        />
      </g>
    );
  });

  const containerClasses = classNames(styles.multipleTranscriptsView, {
    [styles.multipleTranscriptsViewCollapsed]: view === 'collapsed'
  });

  return (
    <div className={containerClasses}>
      {viewToggle}
      <svg width={GRAPHIC_WIDTH}>{renderedTranscripts}</svg>
    </div>
  );
};

const FeatureIdForm = (props: {
  id: string;
  onChange: (id: string) => void;
}) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();

    const value = inputRef.current?.value;

    if (value) {
      props.onChange(value);
    }
  };

  return (
    <form className={styles.form}>
      <p>Enter gene or transcript stable id to change view</p>
      <input ref={inputRef} defaultValue={props.id} />

      <button onClick={handleSubmit}>Get transcripts</button>
    </form>
  );
};

storiesOf('Components|EntityViewer/Transcripts', module).add(
  'unspliced',
  UnsplicedTranscriptsStory
);