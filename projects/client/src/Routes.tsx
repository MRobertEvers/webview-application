import React from 'react';
import { HashRouter, Route } from 'react-router-dom';

import Application from 'src/pages/Application';
import Cubes from 'src/pages/Cubes';

export default function Routes() {
	return (
		<HashRouter>
			<Route path="/cube" component={Cubes} />
			<Route exact path="/" component={Application} />
		</HashRouter>
	);
}
