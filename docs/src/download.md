<script setup>
import { data } from '../.vitepress/data/latest.data.js';
import DownloadTable from '../.vitepress/components/DownloadTable.vue'
</script>

# Download

<DownloadTable :latest="data" />
