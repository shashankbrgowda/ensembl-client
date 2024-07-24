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

import upperFirst from 'lodash/upperFirst';
import type { ReactNode } from 'react';

type Props = {
  type: {
    kind: string;
    value: string;
  } | null;
  fallback?: ReactNode;
  className?: string;
};

const SpeciesType = (props: Props) => {
  const { type: speciesType, fallback = null } = props;

  if (!speciesType && !fallback) {
    return null;
  }

  const content = speciesType ? (
    <span>
      {upperFirst(speciesType.kind)}
      {' - '}
      {speciesType.value}
    </span>
  ) : (
    fallback
  );

  return (
    <span className={props.className} data-part="type">
      {content}
    </span>
  );
};

export default SpeciesType;
