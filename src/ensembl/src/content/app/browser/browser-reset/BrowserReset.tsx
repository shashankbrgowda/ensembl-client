import React, { FunctionComponent, useCallback } from 'react';

import { ChrLocation } from '../browserState';
import { ReactComponent as resetIcon } from 'static/img/browser/track-reset.svg';
import ImageButton, {
  ImageButtonStatus
} from 'src/shared/image-button/ImageButton';

import styles from './BrowserReset.scss';
import { getChrLocationStr } from '../browserHelper';

type BrowserResetProps = {
  activeGenomeId: string;
  activeObjectId: string;
  chrLocation: { [genomeId: string]: ChrLocation };
  defaultChrLocation: { [genomeId: string]: ChrLocation };
  dispatchBrowserLocation: (chrLocation: ChrLocation) => void;
  drawerOpened: boolean;
};

export const BrowserReset: FunctionComponent<BrowserResetProps> = (
  props: BrowserResetProps
) => {
  const {
    activeGenomeId,
    activeObjectId,
    chrLocation,
    defaultChrLocation,
    drawerOpened
  } = props;

  const getResetIconStatus = (): ImageButtonStatus => {
    const chrLocationStr = getChrLocationStr(chrLocation[activeGenomeId]);
    const defaultChrLocationStr = getChrLocationStr(
      defaultChrLocation[activeObjectId]
    );

    if (chrLocationStr === defaultChrLocationStr || drawerOpened === true) {
      return ImageButtonStatus.DISABLED;
    }

    return ImageButtonStatus.ACTIVE;
  };

  const resetBrowser = useCallback(() => {
    if (drawerOpened === true) {
      return;
    }

    props.dispatchBrowserLocation(props.defaultChrLocation[activeObjectId]);
  }, [chrLocation, drawerOpened]);

  return (
    <dd className={styles.resetButton}>
      <div className={styles.imageWrapper}>
        <ImageButton
          buttonStatus={getResetIconStatus()}
          description={'Reset browser image'}
          image={resetIcon}
          onClick={resetBrowser}
          classNames={{ disabled: styles.imageButtonDisabled }}
        />
      </div>
    </dd>
  );
};

export default BrowserReset;
