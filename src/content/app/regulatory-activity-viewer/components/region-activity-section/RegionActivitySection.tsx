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

import { useState, useEffect, useRef, useMemo } from 'react';
import classNames from 'classnames';

import { useAppSelector } from 'src/store';

import prepareFeatureTracks from 'src/content/app/regulatory-activity-viewer/helpers/prepare-feature-tracks/prepareFeatureTracks';

import { getRegionDetailSelectedLocation } from 'src/content/app/regulatory-activity-viewer/state/region-detail/regionDetaillSelectors';

import { useRegionOverviewQuery } from 'src/content/app/regulatory-activity-viewer/state/api/activityViewerApiSlice';

import RegionActivitySectionImage from './RegionActivitySectionImage';

// FIXME: promote these styles to the top level of region activity viewer
import regionOverviewStyles from '../region-overview/RegionOverview.module.css';
import styles from './RegionActivitySection.module.css';

type Props = {
  activeGenomeId: string;
};

const RegionActivitySection = (props: Props) => {
  const { activeGenomeId } = props;
  // TODO: think about how best to handle width changes; maybe they should come from the parent
  const [width, setWidth] = useState(0);
  const regionDetailLocation = useAppSelector((state) =>
    getRegionDetailSelectedLocation(state, activeGenomeId)
  );

  const { currentData } = useRegionOverviewQuery();

  const imageContainerRef = useRef<HTMLDivElement>(null);

  // TODO: width should be recalculated on resize
  // Consider if this is appropriate component for doing this.
  useEffect(() => {
    const imageContainer = imageContainerRef.current as HTMLDivElement;
    const { width: imageContainerWidth } =
      imageContainer.getBoundingClientRect();
    setWidth(imageContainerWidth);
  }, []);

  const preparedData = useMemo(() => {
    if (!currentData) {
      return null;
    }

    // let's consider just a single contiguous slice without "boring" intervals
    const location = currentData.locations[0];

    const selectedStart = regionDetailLocation?.start ?? location.start;
    const selectedEnd = regionDetailLocation?.end ?? location.end;

    const featureTracks = prepareFeatureTracks({
      data: currentData,
      start: selectedStart,
      end: selectedEnd
    });

    return {
      featureTracks,
      start: selectedStart,
      end: selectedEnd
    };
  }, [currentData, regionDetailLocation]);

  const componentClasses = classNames(
    styles.section,
    regionOverviewStyles.grid
  );

  return (
    <div className={componentClasses}>
      <div
        className={regionOverviewStyles.middleColumn}
        ref={imageContainerRef}
      >
        {currentData && preparedData && width && (
          <RegionActivitySectionImage
            width={width}
            regionOverviewData={currentData}
            featureTracks={preparedData.featureTracks}
            start={preparedData.start}
            end={preparedData.end}
          />
        )}
      </div>
    </div>
  );
};

export default RegionActivitySection;
