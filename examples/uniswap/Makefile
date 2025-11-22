deploy:
	. ~/.nvm/nvm.sh && nvm install 20 && nvm use 20
	bash swap_token

front-dir:
	mkdir front

sdk-core:
	. ~/.nvm/nvm.sh && nvm install 18 && nvm use 18
	cd front && \
		git clone -b ethrex_support https://github.com/lambdaclass/sdk-core
	cd front/sdk-core && \
		yarn install && \
		yarn build

smart-order-router:
	cd front && \
		git clone -b ethrex_support https://github.com/lambdaclass/smart-order-router
	. ~/.nvm/nvm.sh && nvm install 18 && nvm use 18 && \
		cd front/smart-order-router && \
		npm install && \
		npm run build

permit2:
	cd front && \
		git clone https://github.com/Uniswap/permit2
	cd front/permit2 && \
		forge install && \
		forge script --broadcast --rpc-url http://localhost:1729 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 script/DeployPermit2.s.sol:DeployPermit2

universal-router:
	cd front && \
		git clone -b ethrex_support --recurse-submodules https://github.com/lambdaclass/universal-router
	cd front/universal-router && \
		. ~/.nvm/nvm.sh && nvm use 18 && \
		npm install && \
		forge install && \
		forge script --broadcast --rpc-url http://localhost:1729 --private-key 0xe4f7dc8b199fdaac6693c9c412ea68aed9e1584d193e1c3478d30a6f01f26057 --sig 'run()' script/deployParameters/DeployEthrexDev.s.sol:DeployEthrexDev

universal-router-sdk:
	cd front && \
		git clone -b ethrex_support https://github.com/lambdaclass/universal-router-sdk
	cd front/universal-router-sdk && \
		. ~/.nvm/nvm.sh && nvm use 18 && \
		yarn install && \
		yarn build

ui-deps: front-dir sdk-core smart-order-router permit2 universal-router universal-router-sdk
	
ui-api:
	cd front && \
		git clone -b ethrex_support https://github.com/lambdaclass/routing-api
	cd front/routing-api && \
		. ~/.nvm/nvm.sh && nvm use 18 && \
		npm install && \
		npm run sls:dev

ui-web:
	cd front && \
		git clone -b ethrex_support https://github.com/lambdaclass/uniswap-interface
	cd front/uniswap-interface && \
		. ~/.nvm/nvm.sh && nvm use 18 && \
		corepack enable && \
		yarn install && \
		yarn web start
