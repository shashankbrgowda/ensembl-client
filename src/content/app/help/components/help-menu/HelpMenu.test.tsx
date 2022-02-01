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

import React from 'react';
import { MemoryRouter } from 'react-router-dom';
import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import HelpMenu, { Props as HelpMenuProps } from './HelpMenu';

import menuData from './helpMenuFixture';

const defaultProps: HelpMenuProps = {
  menu: menuData,
  currentUrl: '/help'
};

const renderMenu = (props: Partial<HelpMenuProps> = {}) =>
  render(
    <MemoryRouter initialEntries={['/help']}>
      <HelpMenu {...defaultProps} {...props} />
    </MemoryRouter>
  );

describe('<HelpMenu>', () => {
  describe('collapsed menu', () => {
    it('renders top-level menu items in the menu bar', () => {
      const { container } = renderMenu();
      const menuBar = container.querySelector('.menuBar') as HTMLElement;
      const topMenuItems = menuData.items.map((item) => item.name);
      const renderedMenuItems = [...menuBar.querySelectorAll('.topMenuItem')];

      expect(
        topMenuItems.every((item) =>
          renderedMenuItems.find((element) => element.textContent === item)
        )
      ).toBe(true);
    });

    it('opens the megamenu when clicked', () => {
      const { container, getByText } = renderMenu();
      expect(container.querySelector('.expandedMenuPanel')).toBeFalsy(); // start with a closed megamenu
      expect(() => getByText('Viewing Ensembl data')).toThrow(); // getByText will throw if it can't find an element
      const itemWithSubmenu = getByText('Using Ensembl');

      userEvent.click(itemWithSubmenu);

      const expandedMenuPanel = container.querySelector('.expandedMenuPanel');
      const submenuItem = getByText('Viewing Ensembl data');

      expect(expandedMenuPanel).toBeTruthy();
      expect(expandedMenuPanel?.contains(submenuItem)).toBe(true);
    });
  });

  describe('expanded menu', () => {
    it('contains expandable submenus', () => {
      const { container, getByText } = renderMenu();
      const itemWithSubmenu = getByText('Using Ensembl');

      userEvent.click(itemWithSubmenu);
      const expandedMenuPanel = container.querySelector(
        '.expandedMenuPanel'
      ) as HTMLElement;

      expect(expandedMenuPanel.querySelectorAll('.submenu').length).toBe(1);

      const submenuItem = getByText('Viewing Ensembl data'); // this item corresponds to a collection of other menu items
      userEvent.hover(submenuItem);

      // hovering over a submenu collection item should expand another submenu
      expect(expandedMenuPanel.querySelectorAll('.submenu').length).toBe(2);
    });
  });
});