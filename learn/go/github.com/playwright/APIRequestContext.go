package playwright

import "github.com/playwright-community/playwright-go"

type APIRequestContext interface {
	Post(url string, options ...playwright.APIRequestContextPostOptions) (playwright.APIResponse, error)
	Put(url string, options ...playwright.APIRequestContextPutOptions) (playwright.APIResponse, error)
	Delete(url string, options ...playwright.APIRequestContextDeleteOptions) (playwright.APIResponse, error)
	Get(url string, options ...playwright.APIRequestContextGetOptions) (playwright.APIResponse, error)
	Dispose(options ...playwright.APIRequestContextDisposeOptions) error
	StorageState(path ...string) (*playwright.StorageState, error)
}
type apiRequestContext struct{ r playwright.APIRequestContext }

func (r *apiRequestContext) Post(url string, options ...playwright.APIRequestContextPostOptions) (playwright.APIResponse, error) {
	return r.r.Post(url, options...)
}
func (r *apiRequestContext) Put(url string, options ...playwright.APIRequestContextPutOptions) (playwright.APIResponse, error) {
	return r.r.Put(url, options...)
}
func (r *apiRequestContext) Delete(url string, options ...playwright.APIRequestContextDeleteOptions) (playwright.APIResponse, error) {
	return r.r.Delete(url, options...)
}
func (r *apiRequestContext) Get(url string, options ...playwright.APIRequestContextGetOptions) (playwright.APIResponse, error) {
	return r.r.Get(url, options...)
}
func (r *apiRequestContext) Dispose(options ...playwright.APIRequestContextDisposeOptions) error {
	return r.r.Dispose(options...)
}
func (r *apiRequestContext) StorageState(path ...string) (*playwright.StorageState, error) {
	return r.r.StorageState(path...)
}
