import React from 'react';
import { mount } from 'enzyme';

import { BrowserNavBarRegionSwitcher } from './BrowserNavBarRegionSwitcher';

import BrowserRegionEditor from '../browser-region-editor/BrowserRegionEditor';
import BrowserRegionField from '../browser-region-field/BrowserRegionField';

jest.mock(
  'src/content/app/browser/browser-region-editor/BrowserRegionEditor',
  () => () => <div>BrowserRegionEditor</div>
);
jest.mock(
  'src/content/app/browser/browser-region-field/BrowserRegionField',
  () => () => <div>BrowserRegionField</div>
);

const props = {
  toggleRegionEditorActive: jest.fn(),
  toggleRegionFieldActive: jest.fn()
};

describe('BrowserNavBarRegionSwitcher', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('renders correctly', () => {
    const wrapper = mount(<BrowserNavBarRegionSwitcher {...props} />);

    expect(wrapper.find(BrowserRegionEditor).length).toBe(1);
    expect(wrapper.find(BrowserRegionField).length).toBe(1);
  });

  it('calls cleanup functions on unmount', () => {
    const wrapper = mount(<BrowserNavBarRegionSwitcher {...props} />);

    expect(props.toggleRegionEditorActive).not.toHaveBeenCalled();
    expect(props.toggleRegionFieldActive).not.toHaveBeenCalled();

    wrapper.unmount();

    expect(props.toggleRegionEditorActive).toHaveBeenCalledWith(false);
    expect(props.toggleRegionFieldActive).toHaveBeenCalledWith(false);
  });
});