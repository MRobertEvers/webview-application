import React from 'react';

import styles from './cube-card.module.css';

interface CubeProps {
	name: string;
}

export default function Cube(props: CubeProps) {
	const { name } = props;
	return (
		<article className={styles['card']}>
			<div className={styles['card-title']}>{name.toUpperCase()}</div>
		</article>
	);
}
