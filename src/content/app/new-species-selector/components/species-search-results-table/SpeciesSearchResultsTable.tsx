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
import classNames from 'classnames';
import upperFirst from 'lodash/upperFirst';

import { formatNumber } from 'src/shared/helpers/formatters/numberFormatter';
import { getSortOrderForColumn } from 'src/content/app/new-species-selector/components/selectable-genomes-table/useOrderedGenomes';

import { Table, ColumnHead } from 'src/shared/components/table';
import Checkbox from 'src/shared/components/checkbox/Checkbox';
import SolidDot from 'src/shared/components/table/dot/SolidDot';
import EmptyDot from 'src/shared/components/table/dot/EmptyDot';
import ExternalLink from 'src/shared/components/external-link/ExternalLink';
import DisabledExternalLink from 'src/shared/components/external-link/DisabledExternalLink';

import type { SpeciesSearchMatch } from 'src/content/app/new-species-selector/types/speciesSearchMatch';
import type { SelectableGenome } from 'src/content/app/new-species-selector/components/selectable-genomes-table/useSelectableGenomesTable';
import type {
  SortRule,
  ChangeSortRule
} from 'src/content/app/new-species-selector/components/selectable-genomes-table/useOrderedGenomes';

import styles from './SpeciesSearchResultsTable.scss';

type Props = {
  isExpanded: boolean;
  results: SelectableGenome[];
  sortRule: SortRule | null;
  onTableExpandToggle: () => void;
  onSpeciesSelectToggle: (
    species: SpeciesSearchMatch,
    isAdding?: boolean
  ) => void;
  onSortRuleChange: ChangeSortRule;
};

const SpeciesSearchResultsTable = (props: Props) => {
  const {
    isExpanded,
    results,
    onSpeciesSelectToggle,
    sortRule,
    onSortRuleChange
  } = props;

  const onSpeciesPreselect = (species: SelectableGenome) => {
    const isAdding = !species.isStaged;
    onSpeciesSelectToggle(species, isAdding);
  };

  return (
    <Table stickyHeader={true} className={styles.table}>
      <thead>
        <tr>
          <ColumnHead>Select to add</ColumnHead>
          <ColumnHead
            sortOrder={getSortOrderForColumn('common_name', sortRule)}
            onSortOrderChange={(newOrder) =>
              onSortRuleChange('common_name', newOrder)
            }
          >
            Common name
          </ColumnHead>
          <ColumnHead
            sortOrder={getSortOrderForColumn('scientific_name', sortRule)}
            onSortOrderChange={(newOrder) =>
              onSortRuleChange('scientific_name', newOrder)
            }
          >
            Scientific name
          </ColumnHead>
          <ColumnHead
            sortOrder={getSortOrderForColumn('type', sortRule)}
            onSortOrderChange={(newOrder) => onSortRuleChange('type', newOrder)}
          >
            Type
          </ColumnHead>
          <ColumnHead
            sortOrder={getSortOrderForColumn('assembly_name', sortRule)}
            onSortOrderChange={(newOrder) =>
              onSortRuleChange('assembly_name', newOrder)
            }
          >
            Assembly
          </ColumnHead>
          <ColumnHead
            sortOrder={getSortOrderForColumn('assembly_accession_id', sortRule)}
            onSortOrderChange={(newOrder) =>
              onSortRuleChange('assembly_accession_id', newOrder)
            }
          >
            Assembly accession
          </ColumnHead>

          <ColumnHead>
            <ShowMore {...props} />
          </ColumnHead>

          {isExpanded && (
            <>
              <ColumnHead
                sortOrder={getSortOrderForColumn(
                  'coding_genes_count',
                  sortRule
                )}
                onSortOrderChange={(newOrder) =>
                  onSortRuleChange('coding_genes_count', newOrder)
                }
              >
                Coding genes
              </ColumnHead>
              <ColumnHead
                sortOrder={getSortOrderForColumn('contig_n50', sortRule)}
                onSortOrderChange={(newOrder) =>
                  onSortRuleChange('contig_n50', newOrder)
                }
              >
                N50
              </ColumnHead>
              <ColumnHead>Variation</ColumnHead>
              <ColumnHead>Regulation</ColumnHead>
              <ColumnHead>Annotation provider</ColumnHead>
              <ColumnHead>Annotation method</ColumnHead>
            </>
          )}
        </tr>
      </thead>
      <tbody>
        {results.map((searchMatch) => (
          <tr
            key={searchMatch.genome_id}
            className={classNames({
              [styles.isAlreadySelected]: searchMatch.isSelected
            })}
          >
            <td>
              <Checkbox
                disabled={searchMatch.isSelected}
                checked={searchMatch.isStaged}
                onChange={() => onSpeciesPreselect(searchMatch)}
              />
            </td>
            <td>{searchMatch.common_name ?? '-'}</td>
            <td>{searchMatch.scientific_name}</td>
            <td>
              <SpeciesType species={searchMatch} />
            </td>
            <td className={styles.assemblyName}>{searchMatch.assembly.name}</td>
            <td>
              {!searchMatch.isSelected ? (
                <ExternalLink
                  to={searchMatch.assembly.url}
                  linkText={searchMatch.assembly.accession_id}
                />
              ) : (
                <DisabledExternalLink>
                  {searchMatch.assembly.accession_id}
                </DisabledExternalLink>
              )}
            </td>

            {/* empty column under the 'show more' heading */}
            <td />

            {isExpanded && (
              <>
                <td>{formatNumber(searchMatch.coding_genes_count)}</td>
                <td>
                  {searchMatch.contig_n50
                    ? formatNumber(searchMatch.contig_n50)
                    : '-'}
                </td>
                <td>
                  {searchMatch.has_variation ? <SolidDot /> : <EmptyDot />}
                </td>
                <td>
                  {searchMatch.has_regulation ? <SolidDot /> : <EmptyDot />}
                </td>
                <td>{searchMatch.annotation_provider}</td>
                <td>{searchMatch.annotation_method}</td>
              </>
            )}
          </tr>
        ))}
      </tbody>
    </Table>
  );
};

const ShowMore = (props: Props) => {
  const { isExpanded, onTableExpandToggle } = props;
  const text = isExpanded ? 'Show less' : 'Show more';

  return (
    <button className={styles.showMore} onClick={onTableExpandToggle}>
      {text}
    </button>
  );
};

const SpeciesType = (props: { species: SpeciesSearchMatch }) => {
  const { type: speciesType, is_reference } = props.species;

  const referenceElement = is_reference ? (
    <span className={styles.referenceGenome}>Reference</span>
  ) : null;

  const speciesTypeText = speciesType
    ? `${upperFirst(speciesType.kind)} - ${speciesType.value}`
    : null;

  if (!referenceElement && !speciesTypeText) {
    return '-';
  }

  return (
    <>
      {speciesTypeText}
      {speciesTypeText && referenceElement && ', '}
      {referenceElement}
    </>
  );
};

export default SpeciesSearchResultsTable;
