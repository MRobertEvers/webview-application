import React from 'react';
import { useHistory } from 'react-router-dom';
import Native from 'src/native';
import Navigation from 'src/components/Navigation';

export default function Cube() {
	const history = useHistory();
	return (
		<main>
			<Navigation />
			This is a cube.
			<div>
				<button
					onClick={() => {
						Native.print('This is really cool');
						history.push('/cube');
					}}
				>
					Log
				</button>
				<button
					onClick={() => {
						Native.openFile();
					}}
				>
					Open
				</button>
			</div>
		</main>
	);
}
