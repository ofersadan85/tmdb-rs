---
name: tmdb-api-expert
description: "Expert on The Movie Database (TMDB) API. Use for TMDB authentication, search, discover, images, movies, TV, people, account/session flows, watchlists, favorites, lists, trending, changes, translations, regions, languages, certifications, append_to_response, exports, and other endpoints documented in developer.themoviedb.org/llms.txt."
---

# TMDB API Expert

## When to Use
Use this skill for any task involving the TMDB API or TMDB documentation, including:
- Choosing the right endpoint for movies, TV, people, collections, images, search, discover, lists, account, auth, watchlists, favorites, trending, changes, translations, certifications, regions, and languages
- Debugging TMDB API requests, auth flows, pagination, filtering, sorting, and response shaping
- Generating code that calls TMDB endpoints correctly
- Explaining TMDB concepts or API behavior using the official documentation
- Mapping user intent to the relevant TMDB reference page in the docs index

## Workflow
1. Identify the request category first: authentication, account/session, media details, discovery/search, images, lists, trending/changes, or utility/reference docs.
2. Consult the relevant TMDB documentation page(s) from the docs index before answering. Treat the linked markdown docs as the source of truth.
3. Verify the auth model and access pattern before giving code or API guidance. Be explicit about whether the task needs a public API key, bearer token, session-based user auth, or write access on behalf of a user.
4. Use official TMDB endpoint paths, parameter names, and field names. Do not invent request parameters or response properties.
5. When the request spans multiple docs, combine the relevant guides with the endpoint reference pages instead of answering from memory.
6. If the user’s goal is implementation, provide the smallest correct example that includes the endpoint, method, auth, required params, and any important query options such as language, region, page, include_adult, append_to_response, or image configuration when relevant.
7. If the data shape is unclear, say which doc page resolves it and recommend checking that page before coding against assumptions.
8. Prefer concise answers that name the specific TMDB concept, the matching docs page, and the practical next step.

## Coverage Notes
Use the TMDB docs index to stay aligned with the official API surface, especially these recurring areas:
- Getting started and authentication
- Search and find-data flows
- Image URL construction and language handling
- Language and region support
- Account, session, favorites, watchlists, rated items, and lists
- Movie, TV, season, episode, person, collection, certification, trending, changes, and translation endpoints
- Append-to-response and other response-shaping helpers

## Output Standards
- Prefer endpoint names and request examples over broad explanations.
- Call out assumptions when the docs leave room for interpretation.
- Keep examples minimal, correct, and easy to adapt.
- If the request depends on a specific TMDB doc page, mention that page by name.
