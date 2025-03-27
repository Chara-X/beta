package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.APIRequestContext]
type APIRequestContext interface {
	Post(url string, options ...playwright.APIRequestContextPostOptions) (playwright.APIResponse, error)
	Put(url string, options ...playwright.APIRequestContextPutOptions) (playwright.APIResponse, error)
	Delete(url string, options ...playwright.APIRequestContextDeleteOptions) (playwright.APIResponse, error)
	Get(url string, options ...playwright.APIRequestContextGetOptions) (playwright.APIResponse, error)
	Dispose(options ...playwright.APIRequestContextDisposeOptions) error
	StorageState(path ...string) (*playwright.StorageState, error)
}
