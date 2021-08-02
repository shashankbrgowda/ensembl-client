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

import React, { useEffect, useState, useRef } from 'react';
import classNames from 'classnames';
import { useDispatch } from 'react-redux';
import { push } from 'connected-react-router';

import { isEnvironment, Environment } from 'src/shared/helpers/environment';

import Chevron from 'src/shared/components/chevron/Chevron';
import HelpMenuLink from './HelpMenuLink';

import {
  Menu as MenuType,
  MenuItem
} from 'src/shared/types/help-and-docs/menu';

import styles from './HelpMenu.scss';

export type Props = {
  menu: MenuType;
  currentUrl: string;
};

const HelpMenu = (props: Props) => {
  const [submenuItems, setSubmenuItems] = useState<MenuItem[] | null>(null);
  const clickedMenuRef = useRef<number | null>(null);

  if (isEnvironment([Environment.PRODUCTION])) {
    return (
      <div className={styles.helpMenu}>
        <div className={styles.menuBar}>Overview</div>
      </div>
    );
  }

  const toggleMegaMenu = (items: MenuItem[], menuIndex: number) => {
    let nextValue = null;
    if (
      clickedMenuRef.current === null ||
      clickedMenuRef.current !== menuIndex
    ) {
      // clicking on a menu item for the first time
      clickedMenuRef.current = menuIndex;
      nextValue = items;
    } else {
      // this means a repeated click on the same menu iteem
      clickedMenuRef.current = null;
    }

    setSubmenuItems(nextValue);
  };

  const closeMegaMenu = () => {
    setSubmenuItems(null);
    clickedMenuRef.current = null;
  };

  const topLevelItems = props.menu.items.map((item, index) => {
    const className = classNames(styles.topMenuItem);
    const commonProps = {
      key: index,
      className
    };
    return item.type === 'collection' ? (
      <span {...commonProps} onClick={() => toggleMegaMenu(item.items, index)}>
        {item.name}
      </span>
    ) : (
      <HelpMenuLink {...commonProps} to={item.url} onClick={closeMegaMenu}>
        {item.name}
      </HelpMenuLink>
    );
  });

  return (
    <div className={styles.helpMenu}>
      <div className={styles.menuBar}>{topLevelItems}</div>
      {submenuItems && (
        <>
          <div className={styles.expandedMenuPanel}>
            <Submenu items={submenuItems} onLinkClick={closeMegaMenu} />
          </div>
          <div
            className={styles.backdrop}
            onMouseEnter={closeMegaMenu}
            onClick={closeMegaMenu}
          />
        </>
      )}
    </div>
  );
};

type SubmenuProps = {
  items: MenuItem[];
  onLinkClick: () => void;
};
const Submenu = (props: SubmenuProps) => {
  const [childItems, setChildItems] = useState<MenuItem[] | null>(null);
  const dispatch = useDispatch();

  useEffect(() => {
    setChildItems(null);
  }, [props.items]);

  const onLinkClick = (url: string) => {
    // hopefully, the url is an internal one;
    // might need extra logic if we can have external urls in the menu
    props.onLinkClick();
    dispatch(push(url));
  };

  const renderedMenuItems = props.items.map((item, index) => {
    const className = classNames(styles.submenuItem);
    const props: Record<string, unknown> = {};
    if (item.type === 'collection') {
      props.onMouseOver = () => setChildItems(item.items);
    } else {
      props.onMouseOver = () => setChildItems(null);
      props.onClick = () => onLinkClick(item.url);
    }
    return (
      <li key={index} {...props} className={className}>
        {item.type === 'collection' ? (
          <>
            {item.name}
            <Chevron direction="right" classNames={{ svg: styles.chevron }} />
          </>
        ) : (
          item.name
        )}
      </li>
    );
  });

  const renderedSubmenu = (
    <ul className={styles.submenu}>{renderedMenuItems}</ul>
  );

  return childItems ? (
    <>
      {renderedSubmenu}
      <Submenu items={childItems} onLinkClick={props.onLinkClick} />
    </>
  ) : (
    renderedSubmenu
  );
};

export default HelpMenu;