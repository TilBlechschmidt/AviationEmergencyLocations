<script>
	import CardDetailView from '$lib/components/guide/CardDetailView.svelte';
	import { _ } from 'svelte-i18n';
</script>

<CardDetailView
	previousLocation="/guide/aircraft"
	previousLabel={$_('guide.aircraft.title')}
	title={$_('guide.limitations.title')}
	nextLocation="/guide/finalWords"
	nextLabel={$_('guide.finalWords.title')}
>
	Even though we try our very best to provide you with the most relevant information for your
	specific situation, it is not possible to create a perfect simulation and we do not happen to have
	supercomputers at hand to run an extensive computational fluid-dynamic analysis to determine the
	impact every last pesky insect on your airframe will make on your glide performance.
	<br />
	<br />
	Jokes aside, here is a number of assumptions and limitations that you have to take into account when
	consuming the data provided:

	<ul class="p-4 list-disc list-inside">
		<li>
			<strong>No wind.</strong> — While wind is favourable on take-off, it might be harmful when
			trying to glide somewhere or attempting a landing with that dreaded tailwind.
			<code>IT IS CRUCIAL</code> that you take a few minutes to figure out how wind will impact the presented
			data!
		</li>
		<li class="pt-4">
			<strong>Best glide.</strong> — All calculations are made at best-glide speed. When turning,
			<a
				href="https://github.com/TilBlechschmidt/ELSA/blob/a9cbe26ad52c4b471139529fbd6f6f6975f0d52d/web-app/elsa/src/data/aircraft.rs#L135"
				target="_blank"
				rel="noopener noreferrer"
				class="border-b border-blue-900 text-blue-900">the glide factor is reduced</a
			> based on the glide angle. However, this only serves as an approximation! Your mileage may vary.
		</li>
		<li class="pt-4">
			<strong>Short field technique.</strong> — For calculation of the landing distances, short
			field technique as described in the POH is assumed. However, landing distances on non-standard
			surfaces like grass or when wet may not be accurate. In accordance with most handbooks, a
			<code>20%</code> increase
			<a
				href="https://github.com/TilBlechschmidt/ELSA/blob/a9cbe26ad52c4b471139529fbd6f6f6975f0d52d/web-app/elsa/src/data/aircraft.rs#L217"
				target="_blank"
				rel="noopener noreferrer"
				class="border-b border-blue-900 text-blue-900">is applied to grass surfaces</a
			>.
		</li>
		<li class="pt-4">
			<strong>Turns included.</strong> — The ranges on the map are worst-case estimates and assume
			that you are facing away from the target location. We do calculate
			<a
				href="https://en.wikipedia.org/wiki/Dubins_path"
				target="_blank"
				rel="noopener noreferrer"
				class="border-b border-blue-900 text-blue-900">the mathematically proven shortest path</a
			>
			given the aircrafts turn radius at your selected bank angle. So if you are actually facing towards
			the target location, there is a bit of headroom included. However, note that this applies
			<code>ONLY</code> to range circles displayed on the map!
		</li>
		<li class="pt-4">
			<strong>Humans factors.</strong> — We do not account for <em>any</em> human factors! It is
			likely that an engine failure may startle you or you forget to put out full flaps for landing.
			Additionally, it is unlikely that you hold the glide speed perfectly or execute a flawless
			turn when your life is at stake. <code>DO NOT TAKE THIS LIGHTLY</code>! You <em>WILL</em> not be
			handling everything perfectly as past examples show.
		</li>
	</ul>

	While everything up to this section has been valuable information, the above points are absolutely
	critical. Make sure you keep everything mentioned here in mind!
</CardDetailView>
