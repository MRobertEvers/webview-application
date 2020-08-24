import React from 'react';
import { HashRouter, Route } from 'react-router-dom';

import Application from 'src/pages/Application';
import Cube from 'src/pages/Cube';

export default function Routes() {
	return (
		<HashRouter>
			<Route path="/cube" component={Cube} />
			<Route exact path="/" component={Application} />
		</HashRouter>
	);
}
