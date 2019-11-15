import React from 'react';
import { mount } from 'enzyme';
import faker from 'faker';

import { TrackPanelList, TrackPanelListProps } from './TrackPanelList';
import TrackPanelListItem from './TrackPanelListItem';

import { createEnsObject } from 'tests/fixtures/ens-object';
import { TrackSet } from '../trackPanelConfig';
import { createGenomeCategories } from 'tests/fixtures/genomes';
import { createTrackStates } from 'tests/fixtures/track-panel';

jest.mock('./TrackPanelListItem', () => () => <div>TrackPanelListItem</div>);

describe('<TrackPanelList />', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  const defaultProps: TrackPanelListProps = {
    activeGenomeId: faker.lorem.words(),
    isDrawerOpened: true,
    launchbarExpanded: true,
    activeEnsObject: createEnsObject(),
    selectedTrackPanelTab: TrackSet.GENOMIC,
    genomeTrackCategories: createGenomeCategories(),
    trackStates: createTrackStates(),
    toggleDrawer: jest.fn(),
    changeDrawerView: jest.fn()
  };

  const mountTrackPanelList = (props?: Partial<TrackPanelListProps>) =>
    mount(<TrackPanelList {...defaultProps} {...props} />);

  describe('rendering', () => {
    test('renders track panel items', () => {
      const wrapper = mountTrackPanelList();
      expect(wrapper.find(TrackPanelListItem).length).toBeGreaterThan(0);
    });

    test('does not render main track if the focus feature is a region', () => {
      const wrapper = mountTrackPanelList({
        activeEnsObject: createEnsObject('region')
      });
      expect(wrapper.find('.mainTrackItem')).toHaveLength(0);
    });
  });
});