import React, { FunctionComponent } from 'react';

import { trackPanelBarConfig, TrackPanelBarItem } from './trackPanelBarConfig';

import TrackPanelBarIcon from './TrackPanelBarIcon';

import chevronLeftIcon from 'static/img/track-panel/chevron-left.svg';
import chevronRightIcon from 'static/img/track-panel/chevron-right.svg';

import styles from './TrackPanelBar.scss';

type TrackPanelBarProps = {
  closeDrawer: () => void;
  drawerOpened: boolean;
  launchbarExpanded: boolean;
  trackPanelOpened: boolean;
  toggleTrackPanel: () => void;
};

const TrackPanelBar: FunctionComponent<TrackPanelBarProps> = (
  props: TrackPanelBarProps
) => {
  const moveTrackPanel = () => {
    if (props.drawerOpened === true) {
      props.closeDrawer();
    } else {
      props.toggleTrackPanel();
    }
  };

  const getClassNames = () => {
    const expandClass: string = props.launchbarExpanded ? '' : styles.expanded;

    return `${styles.trackPanelBar} ${expandClass}`;
  };

  return (
    <div className={getClassNames()}>
      <dl>
        <dt className={styles.sliderButton}>
          <button onClick={moveTrackPanel}>
            {props.trackPanelOpened ? (
              <img src={chevronRightIcon} alt="collapse" />
            ) : (
              <img src={chevronLeftIcon} alt="expand" />
            )}
          </button>
        </dt>
        {trackPanelBarConfig.map((item: TrackPanelBarItem) => (
          <TrackPanelBarIcon key={item.name} iconConfig={item} />
        ))}
      </dl>
    </div>
  );
};

export default TrackPanelBar;
