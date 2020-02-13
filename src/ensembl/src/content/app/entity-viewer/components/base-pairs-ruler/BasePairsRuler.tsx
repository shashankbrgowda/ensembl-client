/*
This component is a ruler for displaying alongside visualisation of a nucleic acid

It follows the following rules for displaying labelled and unlabelled ticks
1. The ruler starts at 1 and ends at the length of the feature.
  Both the start and the end positions of the ruler are labelled.
2. Apart from the start and the end positions, there should be at least one label, but no greater than 5 labels
3. There may also be some unlabelled ticks. The total number of ticks (both labelled and unlabelled)
  between the start and the end positions should not be greater than 10.
4. Last tick cannot be labelled if it is at a less than 10% distance from the end of the ruler
5. Ticks can be either:
  a) multiple of the same power of 10 as the length of the feature, or
  b) half of this power of 10
*/

import React, { useEffect } from 'react';
import { scaleLinear } from 'd3';

import { getTicks } from './basePairsRulerHelper';

import { getCommaSeparatedNumber } from 'src/shared/helpers/numberFormatter';

import styles from './BasePairsRuler.scss';

type Ticks = {
  ticks: number[];
  labelledTicks: number[];
};

type Props = {
  length: number; // number of biological building blocks (e.g. nucleotides) in the feature
  width: number; // number of pixels allotted to the axis on the screen
  onTicksCalculated?: (ticks: Ticks) => void; // way to pass the ticks to the parent if it is interested in them
  standalone: boolean; // wrap the component in an svg element if true
};

const FeatureLengthAxis = (props: Props) => {
  const domain = [1, props.length];
  const range = [0, props.width];
  const scale = scaleLinear()
    .domain(domain)
    .range(range);
  const { ticks, labelledTicks } = getTicks(scale);

  useEffect(() => {
    if (props.onTicksCalculated) {
      props.onTicksCalculated({ ticks, labelledTicks });
    }
  }, [props.length]);

  const renderedAxis = (
    <g>
      <rect
        className={styles.axis}
        x={0}
        y={0}
        width={props.width}
        height={1}
      />
      <g>
        <rect className={styles.tick} width={1} height={6} />
        <text className={styles.label} x={0} y={20} textAnchor="end">
          bp 1
        </text>
      </g>
      {ticks.map((tick) => (
        <g key={tick} transform={`translate(${scale(tick)})`}>
          <rect className={styles.tick} width={1} height={6} />
          {labelledTicks.includes(tick) && (
            <text className={styles.label} x={0} y={20} textAnchor="middle">
              {getCommaSeparatedNumber(tick)}
            </text>
          )}
        </g>
      ))}
      <text
        className={styles.label}
        x={0}
        y={20}
        textAnchor="start"
        transform={`translate(${scale(props.length)})`}
      >
        {getCommaSeparatedNumber(props.length)}
      </text>
    </g>
  );

  return props.standalone ? (
    <svg className={styles.containerSvg} width={props.width}>
      {renderedAxis}
    </svg>
  ) : (
    renderedAxis
  );
};

FeatureLengthAxis.defaultProps = {
  standalone: false
};

export default FeatureLengthAxis;