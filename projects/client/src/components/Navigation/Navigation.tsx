import * as React from 'react';
import { Link } from 'react-router-dom';

import styles from './navigation.module.css';

export default function Navigation(props) {
	return (
		<nav className={styles['navigation']}>
			<Link className={styles['logo']} to="/">
				LiteCube
			</Link>
		</nav>
	);
}
