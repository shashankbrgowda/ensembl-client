import React from 'react';
import { connect } from 'react-redux';
import classNames from 'classnames';

import { getCommaSeparatedNumber } from 'src/shared/helpers/numberFormatter';

import { toggleBrowserNav } from '../browserActions';

import { getActualChrLocation } from '../browserSelectors';
import { getIsDrawerOpened } from '../drawer/drawerSelectors';

import { RootState } from 'src/store';
import { ChrLocation } from '../browserState';

import styles from './BrowserLocationIndicator.scss';

type Props = {
  onClick: () => void;
  location: ChrLocation | null;
  disabled: boolean;
};

export const BrowserLocationIndicator = (props: Props) => {
  const [chrCode, chrStart, chrEnd] = props.location || [];
  if (!chrCode || !chrStart || !chrEnd) {
    return null;
  }

  const className = classNames(styles.browserLocationIndicator, {
    [styles.browserLocationIndicatorDisabled]: props.disabled
  });
  const onClickProps = props.disabled ? {} : { onClick: props.onClick };

  return (
    <div className={className}>
      <div className={styles.chrLabel}>Chromosome</div>
      <div className={styles.chrLocationView} {...onClickProps}>
        <div className={styles.chrCode}>{chrCode}</div>
        <div className={styles.chrRegion}>
          <span>{getCommaSeparatedNumber(chrStart as number)}</span>
          <span className={styles.chrSeparator}>-</span>
          <span>{getCommaSeparatedNumber(chrEnd as number)}</span>
        </div>
      </div>
    </div>
  );
};

const mapStateToProps = (state: RootState) => ({
  location: getActualChrLocation(state),
  disabled: getIsDrawerOpened(state)
});

const mapDispatchToProps = {
  onClick: toggleBrowserNav
};

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(BrowserLocationIndicator);