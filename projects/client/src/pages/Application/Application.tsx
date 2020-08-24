import * as React from 'react';
import { useHistory } from 'react-router-dom';
import Navigation from 'src/components/Navigation';
import Native from 'src/native';

import './application.css';

const Application: React.FunctionComponent = () => {
	const history = useHistory();

	return (
		<div className="application-container">
			<Navigation />
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
		</div>
	);
};

export default Application;
