import React, { useCallback, CSSProperties, useState, useEffect } from 'react';
import analyticsTracking from 'src/services/analytics-service';

import cogOnIcon from 'static/img/shared/cog-on.svg';
import cogOffIcon from 'static/img/shared/cog.svg';
import BrowserTrackConfig from '../browser-track-config/BrowserTrackConfig';
import { useTransition, animated } from 'react-spring';

export type BrowserCogProps = {
  cogActivated: boolean;
  trackId: string;
  updateSelectedCog: (trackId: string | null) => void;
};

const BrowserCog = (props: BrowserCogProps) => {
  const { cogActivated, updateSelectedCog, trackId } = props;

  const toggleCog = useCallback(() => {
    if (cogActivated === false) {
      updateSelectedCog(trackId);

      analyticsTracking.trackEvent({
        category: 'track_settings',
        label: trackId,
        action: 'opened'
      });
    } else {
      updateSelectedCog(null);
    }
  }, [cogActivated]);

  const inline: CSSProperties = { position: 'relative' };
  const imgInline: CSSProperties = {
    height: '18px',
    width: '18px'
  };

  const cogIcon = props.cogActivated ? cogOnIcon : cogOffIcon;

  const [showTrackConfig, setTrackConfigAnimation] = useState(cogActivated);
  useEffect(() => {
    if (cogActivated) {
      setTrackConfigAnimation(true);
      return;
    }
    setTrackConfigAnimation(false);
  }, [cogActivated]);

  const transitions = useTransition(showTrackConfig, null, {
    config: { duration: 100 },
    enter: { opacity: 1 },
    from: { opacity: 0 },
    leave: { opacity: 0 }
  });

  return (
    <>
      <div style={inline}>
        <button onClick={toggleCog}>
          <img src={cogIcon} style={imgInline} alt="Configure track" />
        </button>
      </div>
      {transitions.map(({ item, key, props: style }) => {
        return (
          item && (
            <animated.div key={key} style={style}>
              <BrowserTrackConfig
                onClose={() => props.updateSelectedCog(null)}
              />
            </animated.div>
          )
        );
      })}
    </>
  );
};

export default BrowserCog;