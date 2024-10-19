// @ts-check

import Translate from '@docusaurus/Translate';
import Heading from '@theme/Heading';
import clsx from 'clsx';
import styles from './styles.module.css';

export default function MentionsSection() {
  return (
    <section className={clsx(styles.mediaSection, 'container')}>
      <Heading as="h2" className={styles.title}>
        <Translate>Latest video update</Translate>
      </Heading>
      <div className={clsx('row', styles.media)}>
        <iframe width="100%" height="400px" src="https://www.youtube-nocookie.com/embed/VPYYy1bnw1Y?si=xVDr7GPB1X_gIOuA&amp;controls=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
      </div>
    </section>
  );
}
