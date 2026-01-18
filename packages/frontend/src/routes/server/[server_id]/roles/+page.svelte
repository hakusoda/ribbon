<script lang="ts">
	import { page } from '$app/state';

	import { create_server_roles_query } from '$lib/client/query/server';

	const server_id = page.params.server_id!;
	const server_roles = create_server_roles_query(server_id);
</script>

{#if server_roles.isPending}
	we're loading them roles!!!
{:else if server_roles.isError}
	{server_roles.error}
{:else}
	<h1>Server Roles</h1>
	<table>
		<thead>
			<tr>
				<th>Name</th>
				<td>Requirements</td>
				<td>Member Count</td>
				<td></td>
			</tr>
		</thead>
		<tbody>
			{#each server_roles.data as role}
				<tr>
					<th>{role.name}</th>
					<td>{role.requirements_mock}</td>
					<td>{role.member_count}</td>
					<td>
						<a href={`/server/${server_id}/roles/${role.id}`}>
							edit
						</a>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
{/if}

<style lang="scss">
	a, button {
		all: unset;
		background: #fff;
		border: 1px solid #000;
		color: #000;
		cursor: pointer;
		padding: 4px 8px;
		&:not(:disabled):hover {
			color: blue;
		}
		&:disabled {
			cursor: not-allowed;
			opacity: .5;
		}
	}
	// temporary css i copied from mdn web docsl ol
	table {
		background: white;
		border-collapse: collapse;
		border: 2px solid rgb(140 140 140);
		color: #000;
		font-family: sans-serif;
		font-size: 0.8rem;
		letter-spacing: 1px;
		margin-bottom: 64px;
	}

	thead {
	background-color: rgb(228 240 245);
	}

	th,
	td {
	border: 1px solid rgb(160 160 160);
	padding: 8px 10px;
	}

	td:last-of-type {
	text-align: center;
	}

	tbody > tr:nth-of-type(even) {
	background-color: rgb(237 238 242);
	}
</style>