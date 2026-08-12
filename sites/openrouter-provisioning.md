> ## Documentation Index
>
> Fetch the complete documentation index at: [/docs/llms.txt](/docs/llms.txt)
>
> Use this file to discover all available pages before exploring further.

Skip to main content

[OpenRouter | Documentation home page](https://openrouter.ai)

Search...

⌘KAsk Assistant

  * [Models](https://openrouter.ai/models)
  * [Fusion](https://openrouter.ai/fusion)
  * [Chat](https://openrouter.ai/chat)
  * [Rankings](https://openrouter.ai/rankings)
  * [Apps](https://openrouter.ai/apps)
  * [Docs](https://openrouter.ai/docs)

Search...

Navigation

Authentication

Management API Keys

[Docs](/docs/quickstart)[API Reference](/docs/api_reference/overview)[Client SDKs](/docs/client-sdks/overview)[Agent SDK](/docs/agent-sdk/overview)[Cookbook](/docs/cookbook/get-started/quickstart)

### Overview

  * [Quickstart](/docs/quickstart)
  * [BatchBeta](/docs/batch-quickstart)
  * [Principles](/docs/guides/overview/principles)
  * [Models](/docs/guides/overview/models)
  * [MCP](/docs/guides/overview/mcp-server)
  * Multimodal

  * Authentication

    * [OAuth](/docs/guides/overview/auth/oauth)
    * [Management API Keys](/docs/guides/overview/auth/management-api-keys)
    * [BYOK](/docs/guides/overview/auth/byok)
  * [Stripe Projects](/docs/guides/overview/stripe-projects)
  * [FAQ](/docs/faq)
  * [Report Feedback](/docs/guides/overview/report-feedback)

### Models & Routing

  * [Model Fallbacks](/docs/guides/routing/model-fallbacks)
  * [Provider Selection](/docs/guides/routing/provider-selection)
  * [Auto Exacto](/docs/guides/routing/auto-exacto)
  * [Private Models](/docs/guides/routing/private-models)
  * Model Variants

  * Routers

### Features

  * [Workspaces](/docs/guides/features/workspaces)
  * [Workspace Budgets](/docs/guides/features/workspaces/workspace-budgets)
  * [Switching Workspaces](/docs/guides/features/workspaces/switching)
  * [Single Sign-On (SSO)](/docs/guides/features/sso)
  * [SCIM Group Mappings](/docs/guides/features/scim-mappings)
  * [Presets](/docs/guides/features/presets)
  * [Custom Classifiers](/docs/guides/features/classifiers)
  * [Response Caching](/docs/guides/features/response-caching)
  * [Tool Calling](/docs/guides/features/tool-calling)
  * Server Tools

  * Plugins

  * [Structured Outputs](/docs/guides/features/structured-outputs)
  * [Message Transforms](/docs/guides/features/message-transforms)
  * [Zero Completion Insurance](/docs/guides/features/zero-completion-insurance)
  * [ZDR](/docs/guides/features/zdr)
  * [App Attribution](/docs/app-attribution)
  * Guardrails

  * [Service Tiers](/docs/guides/features/service-tiers)
  * [Sovereign AI](/docs/guides/features/sovereign-ai)
  * [Router Metadata](/docs/guides/features/router-metadata)
  * [Input & Output Logging](/docs/guides/features/input-output-logging)
  * Broadcast

### Ori

  * [Ori Eval](/docs/guides/ori/eval)
  * [Ori Harness](/docs/guides/ori/harness)
  * [Where Ori writes files](/docs/guides/ori/files)

### Privacy

  * [Data Collection](/docs/guides/privacy/data-collection)
  * [Provider Logging](/docs/guides/privacy/provider-logging)

### Best Practices

  * [Latency and Performance](/docs/guides/best-practices/latency-and-performance)
  * [Prompt Caching](/docs/guides/best-practices/prompt-caching)
  * [Uptime Optimization](/docs/guides/best-practices/uptime-optimization)
  * [Reasoning Tokens](/docs/guides/best-practices/reasoning-tokens)

### Community

  * [For Providers](/docs/guides/community/for-providers)
  * [Frameworks and Integrations Overview](/docs/guides/community/frameworks-and-integrations-overview)
  * [Awesome OpenRouter](/docs/guides/community/awesome-openrouter)
  * [Effect AI SDK](/docs/guides/community/effect-ai-sdk)
  * [Arize AX](/docs/guides/community/arize)
  * [LangChain](/docs/guides/community/langchain)
  * [LiveKit](/docs/guides/community/livekit)
  * [Langfuse](/docs/guides/community/langfuse)
  * [Mastra](/docs/guides/community/mastra)
  * [OpenAI SDK](/docs/guides/community/openai-sdk)
  * [Anthropic Agent SDK](/docs/guides/community/anthropic-agent-sdk)
  * [PydanticAI](/docs/guides/community/pydantic-ai)
  * [Replit](/docs/guides/community/replit)
  * [TanStack AI](/docs/guides/community/tanstack-ai)
  * [Vercel AI SDK](/docs/guides/community/vercel-ai-sdk)
  * [Xcode](/docs/guides/community/xcode)
  * [Zapier](/docs/guides/community/zapier)
  * [Infisical](/docs/guides/community/infisical)

## On this page

  * Creating a Management API Key
  * Use Cases
  * Example Usage
  * Response Format

Authentication

# Management API Keys

Copy pageCopy page

Manage API keys programmatically

Copy pageCopy page

OpenRouter provides endpoints to programmatically manage your API keys, enabling key creation and management for applications that need to distribute or rotate keys automatically.

##

​

Creating a Management API Key

To use the key management API, you first need to create a Management API key:

  1. Go to the [Management API Keys page](https://openrouter.ai/settings/management-keys)
  2. Click “Create New Key”
  3. Complete the key creation process

Management keys cannot be used to make API calls to OpenRouter’s completion endpoints - they are exclusively for administrative operations.

##

​

Use Cases

Common scenarios for programmatic key management include:

  * **SaaS Applications** : Automatically create unique API keys for each customer instance
  * **Key Rotation** : Regularly rotate API keys for security compliance
  * **Usage Monitoring** : Track key usage and automatically disable keys that exceed limits (with optional daily/weekly/monthly limit resets)

##

​

Example Usage

All key management endpoints are under `/api/v1/keys` and require a Management API key in the Authorization header.

TypeScript SDK

Python

TypeScript (fetch)

    import { OpenRouter } from '@openrouter/sdk';

    const openRouter = new OpenRouter({
      apiKey: 'your-management-key', // Use your Management API key
    });

    // List the most recent 100 API keys
    const keys = await openRouter.apiKeys.list();

    // You can paginate using the offset parameter
    const keysPage2 = await openRouter.apiKeys.list({ offset: 100 });

    // Create a new API key
    const newKey = await openRouter.apiKeys.create({
      name: 'Customer Instance Key',
      limit: 1000, // Optional credit limit
    });

    // Get a specific key
    const keyHash = '<YOUR_KEY_HASH>';
    const key = await openRouter.apiKeys.get(keyHash);

    // Update a key
    const updatedKey = await openRouter.apiKeys.update(keyHash, {
      name: 'Updated Key Name',
      disabled: true, // Optional: Disable the key
      includeByokInLimit: false, // Optional: control BYOK usage in limit
      limitReset: 'daily', // Optional: reset limit every day at midnight UTC
    });

    // Delete a key
    await openRouter.apiKeys.delete(keyHash);

See all 32 lines

    import requests

    MANAGEMENT_API_KEY = "your-management-key"
    BASE_URL = "https://openrouter.ai/api/v1/keys"

    # List the most recent 100 API keys
    response = requests.get(
        BASE_URL,
        headers={
            "Authorization": f"Bearer {MANAGEMENT_API_KEY}",
            "Content-Type": "application/json"
        }
    )

    # You can paginate using the offset parameter
    response = requests.get(
        f"{BASE_URL}?offset=100",
        headers={
            "Authorization": f"Bearer {MANAGEMENT_API_KEY}",
            "Content-Type": "application/json"
        }
    )

    # Create a new API key
    response = requests.post(
        f"{BASE_URL}/",
        headers={
            "Authorization": f"Bearer {MANAGEMENT_API_KEY}",
            "Content-Type": "application/json"
        },
        json={
            "name": "Customer Instance Key",
            "limit": 1000  # Optional credit limit
        }
    )

    # Get a specific key
    key_hash = "<YOUR_KEY_HASH>"
    response = requests.get(
        f"{BASE_URL}/{key_hash}",
        headers={
            "Authorization": f"Bearer {MANAGEMENT_API_KEY}",
            "Content-Type": "application/json"
        }
    )

    # Update a key
    response = requests.patch(
        f"{BASE_URL}/{key_hash}",
        headers={
            "Authorization": f"Bearer {MANAGEMENT_API_KEY}",
            "Content-Type": "application/json"
        },
        json={
            "name": "Updated Key Name",
            "disabled": True,  # Optional: Disable the key
            "include_byok_in_limit": False,  # Optional: control BYOK usage in limit
            "limit_reset": "daily"  # Optional: reset limit every day at midnight UTC
        }
    )

    # Delete a key
    response = requests.delete(
        f"{BASE_URL}/{key_hash}",
        headers={
            "Authorization": f"Bearer {MANAGEMENT_API_KEY}",
            "Content-Type": "application/json"
        }
    )

    const MANAGEMENT_API_KEY = 'your-management-key';
    const BASE_URL = 'https://openrouter.ai/api/v1/keys';

    // List the most recent 100 API keys
    const listKeys = await fetch(BASE_URL, {
      headers: {
        Authorization: `Bearer ${MANAGEMENT_API_KEY}`,
        'Content-Type': 'application/json',
      },
    });

    // You can paginate using the `offset` query parameter
    const listKeys = await fetch(`${BASE_URL}?offset=100`, {
      headers: {
        Authorization: `Bearer ${MANAGEMENT_API_KEY}`,
        'Content-Type': 'application/json',
      },
    });

    // Create a new API key
    const createKey = await fetch(`${BASE_URL}`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${MANAGEMENT_API_KEY}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name: 'Customer Instance Key',
        limit: 1000, // Optional credit limit
      }),
    });

    // Get a specific key
    const keyHash = '<YOUR_KEY_HASH>';
    const getKey = await fetch(`${BASE_URL}/${keyHash}`, {
      headers: {
        Authorization: `Bearer ${MANAGEMENT_API_KEY}`,
        'Content-Type': 'application/json',
      },
    });

    // Update a key
    const updateKey = await fetch(`${BASE_URL}/${keyHash}`, {
      method: 'PATCH',
      headers: {
        Authorization: `Bearer ${MANAGEMENT_API_KEY}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name: 'Updated Key Name',
        disabled: true, // Optional: Disable the key
        include_byok_in_limit: false, // Optional: control BYOK usage in limit
        limit_reset: 'daily', // Optional: reset limit every day at midnight UTC
      }),
    });

    // Delete a key
    const deleteKey = await fetch(`${BASE_URL}/${keyHash}`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${MANAGEMENT_API_KEY}`,
        'Content-Type': 'application/json',
      },
    });

##

​

Response Format

API responses return JSON objects containing key information:

    {
      "data": [
        {
          "created_at": "2025-02-19T20:52:27.363244+00:00",
          "updated_at": "2025-02-19T21:24:11.708154+00:00",
          "hash": "<YOUR_KEY_HASH>",
          "label": "sk-or-v1-abc...123",
          "name": "Customer Key",
          "disabled": false,
          "limit": 10,
          "limit_remaining": 10,
          "limit_reset": null,
          "include_byok_in_limit": false,
          "usage": 0,
          "usage_daily": 0,
          "usage_weekly": 0,
          "usage_monthly": 0,
          "byok_usage": 0,
          "byok_usage_daily": 0,
          "byok_usage_weekly": 0,
          "byok_usage_monthly": 0
        }
      ]
    }

See all 24 lines

When creating a new key, the response will include the key string itself. Read more in the [API reference](/docs/api/api-reference/api-keys/create-a-new-api-key).

[OAuth](/docs/guides/overview/auth/oauth)[BYOK](/docs/guides/overview/auth/byok)

⌘I

Assistant

Responses are generated using AI and may contain mistakes.