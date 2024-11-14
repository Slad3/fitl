import{a as l,t as n}from"../chunks/disclose-version.mL42o9Kk.js";import"../chunks/legacy.6wHpMh_u.js";import{m as i,k as a,F as h,l as s}from"../chunks/runtime.kV6Rt6fy.js";import{s as c}from"../chunks/attributes.np55ldqT.js";const p=""+new URL("../assets/spotifySearchImage.DwjxwRHI.jpg",import.meta.url).href;var m=n(`<article class="prose-gray prose-base text-white"><h1>Why FiTL Over Current/General Search Algorithms</h1> <div><h2>Advantages Over a General Search Algorithm</h2> <p>Why develop and/or learn an entire language just to do a more specific search?</p> <p>In my experience with typical applications (especially ones on the web), search works ever so
			slightly differently on each site. While the inconsistency is usually the biggest problem, the
			leeway when it comes to searching/filtering down a table can get frustrating if not confusing.
			While search acceptance is based on the use case of the application, underdeveloped
			applications (IE old product pages) may provide a search that is too strict and will not catch
			many obvious cases.</p> <p>Many modern applications (Spotify...) however may provide a search that either catches not
			only too many cases, or also catches strange cases where you are left wondering how it got
			that result from that search query. For example, why am I getting "the Way You Make Me Feel"
			by Michael Jackson and "How Do I Make You Love Me" by The Weeknd from the query: <b>"you the make"</b>? "the" is not even in the title "How Do I Make You Love Me"?</p> <img alt="Spotify's Odd Filtering Algorithm"> <p>The Simple answer is that it doesn't get just search for exact (or close matches) in song
			titles, rather it super fuzzy searches within all fields. An algorithm that works for the most
			part most of the time, but is very difficult to get specific, especially when I just want to
			quickly make a temp playlist of all my liked songs from a specific artist right before I hit
			the road.</p> <p>-- Spotify Rant Over --</p> <p>This is not something specifically directed at Spotify as many other modern services (YouTube,
			Reddit, Genius.com, etc, all share similar problems), Spotify is just where I filter tables
			data the most.</p> <h2>So how does FiTL aim to fix the overall issue?</h2> <ol class=" space-y-1 list-disc list-outside"><li>FiTL allows the same kind of quick fuzzy filtering, but also allows for more specific
				filtering</li> <li>FiTL's syntax will stay the same across applications (granted they implement the library),
				leaving less room for guessing and unexpected behavior for searches and filtering</li> <li>Compared to writing your own filtering algorithm, FiTL is more efficient to implement and
				more efficient to run, since the library is either compiled into raw binary, or into web
				assembly keeping it lightweight and responsive</li> <li>If all else, FiTL can run along side your own search/filtering algorithm by allowing users
				to further filter down search results</li></ol></div></article>`);function f(o){var e=m(),t=i(a(e),2),r=i(a(t),8);c(r,"src",p),h(10),s(t),s(e),l(o,e)}export{f as component};
