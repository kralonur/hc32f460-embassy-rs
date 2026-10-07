#[doc = "Register `TXBUF` writer"]
pub type W = crate::W<TxbufSpec>;
impl core::fmt::Debug for crate::generic::Reg<TxbufSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "desc TXBUF\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbuf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbufSpec;
impl crate::RegisterSpec for TxbufSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txbuf::W`](W) writer structure"]
impl crate::Writable for TxbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBUF to value 0"]
impl crate::Resettable for TxbufSpec {}
