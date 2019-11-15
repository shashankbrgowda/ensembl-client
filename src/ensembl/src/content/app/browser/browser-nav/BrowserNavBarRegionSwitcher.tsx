import React, { useEffect } from 'react';
import { connect } from 'react-redux';

import BrowserRegionEditor from '../browser-region-editor/BrowserRegionEditor';
import BrowserRegionField from '../browser-region-field/BrowserRegionField';

import {
  toggleRegionEditorActive,
  toggleRegionFieldActive
} from '../browserActions';

import styles from './BrowserNavBarRegionSwitcher.scss';

type Props = {
  toggleRegionEditorActive: (isActive: boolean) => void;
  toggleRegionFieldActive: (isActive: boolean) => void;
};

export const BrowserNavBarRegionSwitcher = (props: Props) => {
  // cleanup on unmount
  useEffect(
    () => () => {
      props.toggleRegionEditorActive(false);
      props.toggleRegionFieldActive(false);
    },
    []
  );

  return (
    <div className={styles.regionSwitcher}>
      <div className={styles.regionFieldWrapper}>
        <BrowserRegionField />
      </div>
      <div className={styles.regionEditorWrapper}>
        <BrowserRegionEditor />
      </div>
    </div>
  );
};

const mapDispatchToProps = {
  toggleRegionEditorActive,
  toggleRegionFieldActive
};

export default connect(
  null,
  mapDispatchToProps
)(BrowserNavBarRegionSwitcher);