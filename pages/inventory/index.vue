<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { CalendarIcon } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const route = useRoute();
const { t, d } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();
const searchQuery = ref(route.query.search as string);
const transaction_type = ref(route.query.transaction_type as string);
const created_at = ref(route.query.created_at as string);

const LIMIT = 25;

const queryParams = computed<QueryParams>(() => ({
  search: route.query.search,
  page: route.query.page,
  refresh: route.query.refresh,
  limit: route.query.limit,
  transaction_type: route.query.transaction_type,
  created_at: route.query.created_at,
}));

async function fetchInventory() {
  try {
    const res: Res<any> = await invoke("list_inventory", {
      args: {
        search: queryParams.value.search ?? "",
        page: Number(queryParams.value.page) ?? 1,
        limit: queryParams.value.limit
          ? Number(queryParams.value.limit)
          : LIMIT,
        status: queryParams.value.transaction_type,
        created_at: queryParams.value.created_at,
      },
    });
    return res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST INVENTORY: ${err.error ? err.error : err.message}`);
    return { inventory: [], count: 0 };
  }
}

const { data } = useAsyncData(fetchInventory, { watch: [queryParams] });

const inventory = computed<InventoryT[]>(() => data.value?.inventory ?? []);
const totalRows = computed<number>(() => data.value?.count ?? 0);

provide("count", totalRows);
provide(
  "itemsPerPage",
  queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT
);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value });
}, 500);

watch(searchQuery, debouncedSearch);

watch([transaction_type, created_at], () => {
  updateQueryParams({
    transaction_type: transaction_type.value,
    page: 1,
    created_at: created_at.value
      ? new Date(created_at.value).toISOString()
      : undefined,
  });
});
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <div class="flex justify-between w-full gap-9 mb-2">
        <div class="w-full grid grid-cols-3 gap-2 lg:max-w-screen-lg">
          <Input v-model="searchQuery" type="text" :placeholder="t('search')" />
          <Popover>
            <PopoverTrigger as-child>
              <Button
                variant="outline"
                :class="
                  cn(
                    'w-full justify-start text-left font-normal',
                    !created_at && 'text-muted-foreground',
                  )
                "
              >
                <CalendarIcon class="mr-2 h-4 w-4" />
                <span class="text-nowrap">{{
                  created_at ? d(new Date(created_at), "short") : t("pick-date")
                }}</span>
              </Button>
            </PopoverTrigger>
            <PopoverContent class="w-auto p-0">
              <Calendar v-model="created_at" />
            </PopoverContent>
          </Popover>
          <Select v-model="transaction_type">
            <SelectTrigger>
              <SelectValue
                class="text-muted-foreground"
                :placeholder="t('select-status')"
              />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="OUT">
                {{ t("status.out") }}
              </SelectItem>
              <SelectItem value="IN">
                {{ t("status.in") }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div />
      </div>
      <InventoryTable :inventory="inventory" />
    </div>
  </main>
</template>
