<template>
    <div>
        <h2>Check Balance</h2>
        <input v-model="address" placeholder="Enter wallet address" />
        <button @click="fetchBalance">Check Balance</button>
        <p v-if="balance !== null">Balance: {{ balance }}</p>
    </div>
</template>

<script>
import { getBalance } from '@/services/api';

export default {
    data() {
        return {
            address: '',
            balance: null,
        };
    },
    methods: {
        async fetchBalance() {
            this.balance = await getBalance(this.address);
        },
    },
};
</script>

<style scoped>
/* Add your styles here */
</style>
