import codegen from '@cosmwasm/ts-codegen';

codegen({
  contracts: [
    {
      name: 'InjectiveInterchainPersona',
      dir: '../schema'
    }
  ],
  outPath: './types/',

  // options are completely optional ;)
  options: {
    bundle: {
      bundleFile: 'index.ts',
      scope: 'contracts'
    },
    types: {
      enabled: true
    },
    client: {
      enabled: true
    },
    reactQuery: {
      enabled: false,
      optionalClient: false,
      version: 'v4',
      mutations: false,
      queryKeys: false,
      queryFactory: false,
    },
    recoil: {
      enabled: false
    },
    messageComposer: {
      enabled: true
    },
    messageBuilder: {
      enabled: false
    },
    useContractsHook: {
      enabled: false
    }
  }
}).then(() => {
  console.log('✨ all done!');
});
