import * as React from 'react';
import Native from '../native';

import './application.css';

const Application: React.FunctionComponent = () => {
	return (
		<div className="application-container">
			<div className="application-header">
				<h2>TITLE</h2>
			</div>
			<div>
				<button
					onClick={() => {
						Native.print('This is really cool');
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
