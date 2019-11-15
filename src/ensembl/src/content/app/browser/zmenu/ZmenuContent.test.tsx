import React from 'react';
import { Provider } from 'react-redux';
import { mount } from 'enzyme';
import faker from 'faker';

import {
  ZmenuContent,
  ZmenuContentProps,
  ZmenuContentItem,
  ZmenuContentItemProps,
  ZmenuContentLine
} from './ZmenuContent';

import {
  Markup,
  ZmenuContentItem as ZmenuContentItemType
} from './zmenu-types';
import { createZmenuContent } from 'tests/fixtures/browser';
import configureStore from 'src/store';

describe('<ZmenuContent />', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  const store = configureStore();
  const wrappingComponent = (props: any) => (
    <Provider store={store}>{props.children}</Provider>
  );
  const defaultProps: ZmenuContentProps = {
    content: createZmenuContent()
  };
  let wrapper: any;

  beforeEach(() => {
    wrapper = mount(<ZmenuContent {...defaultProps} />, { wrappingComponent });
  });

  describe('rendering', () => {
    test('renders the correct zmenu content information', () => {
      const firstLineData = wrapper
        .find(ZmenuContentLine)
        .first()
        .props()
        .blocks.map((items: ZmenuContentItemType[]) => items[0].text);

      firstLineData.forEach((lineText: string) => {
        expect(
          wrapper
            .find('.zmenuContentLine')
            .first()
            .text()
        ).toContain(lineText);
      });
    });
  });

  describe('behaviour', () => {
    test('changes focus feature when feature link is clicked', () => {
      const props: ZmenuContentItemProps = {
        id: faker.lorem.words(),
        markup: [Markup.FOCUS],
        text: faker.lorem.words(),
        changeFocusObject: jest.fn()
      };
      const wrapper = mount(<ZmenuContentItem {...props} />);

      wrapper.simulate('click');
      expect(wrapper.props().changeFocusObject).toHaveBeenCalledTimes(1);
    });
  });
});