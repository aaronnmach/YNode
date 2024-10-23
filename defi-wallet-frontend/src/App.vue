<template>
  <div id="app">
    <Wallet />
    <Balance />
    <Transaction />
  </div>
</template>

<script>
import Wallet from './components/WalletComponent.vue';
import Balance from './components/BalanceComponent.vue';
import Transaction from './components/TransactionComponent.vue';

export default {
  components: {
    Wallet,
    Balance,
    Transaction,
  },
};
</script>

<style>
/* Add global styles here */
</style>
