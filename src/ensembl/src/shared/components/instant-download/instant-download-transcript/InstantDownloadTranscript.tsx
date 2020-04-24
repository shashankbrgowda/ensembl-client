import React, { useState, useEffect } from 'react';
import classNames from 'classnames';
import pick from 'lodash/pick';

import { fetchForTranscript } from '../instant-download-fetch/fetchForTranscript';

import InstantDownloadTranscriptVisualisation from './InstantDownloadTranscriptVisualisation';
import Checkbox from 'src/shared/components/checkbox/Checkbox';
import InstantDownloadButton from '../instant-download-button/InstantDownloadButton';

import styles from './InstantDownloadTranscript.scss';

type Layout = 'horizontal' | 'vertical';

type TranscriptFields = {
  id: string;
  so_term: string;
};

type GeneFields = {
  id: string;
};

type Props = {
  transcript: TranscriptFields;
  gene: GeneFields;
  layout: Layout;
};

type TranscriptSectionProps = {
  transcript: TranscriptFields;
  options: Partial<TranscriptOptions>;
  onChange: (key: keyof TranscriptOptions) => void;
};

type GeneSectionProps = {
  gene: GeneFields;
  isGenomicSequenceSelected: boolean;
  onChange: () => void;
};

export type TranscriptOptions = {
  genomicSequence: boolean;
  proteinSequence: boolean;
  cdna: boolean;
  cds: boolean;
};

export type TranscriptOption = keyof Partial<TranscriptOptions>;

export const transcriptOptionsOrder: TranscriptOption[] = [
  'genomicSequence',
  'cdna',
  'proteinSequence',
  'cds'
];

const defaultTranscriptOptions: TranscriptOptions = {
  genomicSequence: false,
  cdna: false,
  proteinSequence: false,
  cds: false
};

const transcriptOptionLabels: Record<keyof TranscriptOptions, string> = {
  genomicSequence: 'Genomic sequence',
  proteinSequence: 'Protein sequence',
  cdna: 'cDNA',
  cds: 'CDS'
};

const filterTranscriptOptions = (
  so_term: string
): Partial<TranscriptOptions> => {
  return so_term === 'protein_coding'
    ? defaultTranscriptOptions
    : pick(defaultTranscriptOptions, ['genomicSequence', 'cdna']);
};

const InstantDownloadTranscript = (props: Props) => {
  const {
    transcript: { so_term }
  } = props;
  const [transcriptOptions, setTranscriptOptions] = useState(
    filterTranscriptOptions(so_term)
  );
  const [isGeneSequenceSelected, setIsGeneSequenceSelected] = useState(false);

  useEffect(() => {
    setTranscriptOptions(filterTranscriptOptions(so_term));
  }, [so_term]);

  const onTranscriptOptionChange = (key: keyof TranscriptOptions) => {
    const updatedOptions = {
      ...transcriptOptions,
      [key]: !transcriptOptions[key]
    };
    setTranscriptOptions(updatedOptions);
  };
  const onGeneOptionChange = () => {
    setIsGeneSequenceSelected(!isGeneSequenceSelected);
  };
  const onSubmit = () => {
    const payload = {
      transcriptId: props.transcript.id,
      geneId: props.gene.id,
      options: {
        transcript: transcriptOptions,
        gene: { genomicSequence: isGeneSequenceSelected }
      }
    };
    fetchForTranscript(payload);
  };

  const layoutClass =
    props.layout === 'horizontal'
      ? classNames(styles.layout, styles.layoutHorizontal)
      : classNames(styles.layout, styles.layoutVertical);

  const isButtonDisabled = !hasSelectedOptions({
    ...transcriptOptions,
    geneSequence: isGeneSequenceSelected
  });

  return (
    <div className={layoutClass}>
      <TranscriptSection
        transcript={props.transcript}
        options={transcriptOptions}
        onChange={onTranscriptOptionChange}
      />
      <GeneSection
        gene={props.gene}
        isGenomicSequenceSelected={isGeneSequenceSelected}
        onChange={onGeneOptionChange}
      />
      <InstantDownloadButton
        className={isButtonDisabled ? styles.downloadButtonDisabled : undefined}
        isDisabled={isButtonDisabled}
        onClick={onSubmit}
      />
    </div>
  );
};

InstantDownloadTranscript.defaultProps = {
  layout: 'horizontal'
} as Props;

const TranscriptSection = (props: TranscriptSectionProps) => {
  const { transcript, options } = props;
  const checkboxes = transcriptOptionsOrder.map((key) => (
    <Checkbox
      key={key}
      classNames={{ unchecked: styles.checkboxUnchecked }}
      labelClassName={styles.checkboxLabel}
      label={transcriptOptionLabels[key as TranscriptOption]}
      checked={options[key as TranscriptOption] as boolean}
      onChange={() => props.onChange(key as TranscriptOption)}
    />
  ));

  const transcriptVisualisation = (
    <InstantDownloadTranscriptVisualisation
      isGenomicSequenceEnabled={options.genomicSequence}
      isProteinSequenceEnabled={options.proteinSequence}
      isCDNAEnabled={options.cdna}
      isCDSEnabled={options.cds}
    />
  );

  return (
    <div className={styles.transcriptSection}>
      <div className={styles.label}>
        Transcript
        <span className={styles.featureId}>{transcript.id}</span>
      </div>
      <div className={styles.transcriptVis}>{transcriptVisualisation}</div>
      <div className={styles.checkboxGrid}>{checkboxes}</div>
    </div>
  );
};

const GeneSection = (props: GeneSectionProps) => {
  return (
    <div className={styles.geneSection}>
      <div className={styles.label}>
        Gene
        <span className={styles.featureId}>{props.gene.id}</span>
      </div>
      <div>
        <Checkbox
          classNames={{
            checkboxHolder: styles.checkboxWrapper,
            unchecked: styles.checkboxUnchecked
          }}
          labelClassName={styles.checkboxLabel}
          label="Genomic sequence"
          checked={props.isGenomicSequenceSelected}
          onChange={props.onChange}
        />
      </div>
    </div>
  );
};

const hasSelectedOptions = (
  options: Partial<TranscriptOptions> & { geneSequence: boolean }
) => {
  return Object.keys(options).some(
    (key) => options[key as keyof TranscriptOptions]
  );
};

export default InstantDownloadTranscript;