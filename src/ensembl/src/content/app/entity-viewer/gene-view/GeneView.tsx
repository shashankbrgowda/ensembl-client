import React from 'react';
import { connect } from 'react-redux';
import { useQuery } from '@apollo/react-hooks';
import { gql } from 'apollo-boost';

import { getEntityViewerActiveEnsObject } from 'src/content/app/entity-viewer/state/general/entityViewerGeneralSelectors';

import GeneOverviewImage from './components/gene-overview-image/GeneOverviewImage';

import { Gene } from 'src/content/app/entity-viewer/types/gene';
import { RootState } from 'src/store';

import styles from './GeneView.scss';

type GeneViewProps = {
  geneId: string | null;
};

type GeneViewWithDataProps = {
  gene: Gene;
};

const QUERY = gql`
  query Gene($id: String!) {
    gene(byId: { id: $id }) {
      id
      slice {
        location {
          start
          end
        }
        region {
          strand {
            code
          }
        }
      }
      transcripts {
        slice {
          location {
            start
            end
          }
        }
        exons {
          slice {
            location {
              start
              end
            }
          }
        }
      }
    }
  }
`;

const GeneView = (props: GeneViewProps) => {
  const { data } = useQuery<{ gene: Gene }>(QUERY, {
    variables: { id: props.geneId },
    skip: !props.geneId
  });

  // TODO decide about the loader and possibly about error handling

  if (!data) {
    return null;
  }

  return <GeneViewWithData gene={data.gene} />;
};

const GeneViewWithData = (props: GeneViewWithDataProps) => {
  return (
    <div className={styles.geneView}>
      <div className={styles.featureImage}>
        <GeneOverviewImage gene={props.gene} />
      </div>
      <div className={styles.viewInLinks}>View in GB</div>

      <div className={styles.geneViewTabs}>
        These are the Entity Viewer tabs...
      </div>
      <div className={styles.geneViewTable}>
        This is the Entity Viewer table...
      </div>
    </div>
  );
};

const mapStateToProps = (state: RootState) => ({
  // FIXME: this will have to be superseded with a proper way we get ids
  geneId: getEntityViewerActiveEnsObject(state)?.stable_id || null
});

export default connect(mapStateToProps)(GeneView);