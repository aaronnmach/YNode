<template>
    <div>
        <h2>Create Wallet</h2>
        <button @click="createNewWallet">Create Wallet</button>
        <p v-if="walletAddress">Wallet Address: {{ walletAddress }}</p>
    </div>
</template>

<script>
import { createWallet } from '@/services/api';

export default {
    data() {
        return {
            walletAddress: '',
        };
    },
    methods: {
        async createNewWallet() {
            const data = await createWallet();
            this.walletAddress = data; // Adjust based on the API response
        },
    },
};
</script>

<style scoped>
/* Add your styles here */
</style>
