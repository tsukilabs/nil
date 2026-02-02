<script setup>
import { useServer } from '../.vitepress/composables/useServer';
import ServerStatus from '../.vitepress/components/ServerStatus.vue';

const server = useServer();
</script>

# Server Status

<ServerStatus v-if="server" :server />
