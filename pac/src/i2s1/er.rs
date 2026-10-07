#[doc = "Register `ER` reader"]
pub type R = crate::R<ErSpec>;
#[doc = "Register `ER` writer"]
pub type W = crate::W<ErSpec>;
#[doc = "Field `TXERR` reader - desc TXERR"]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - desc TXERR"]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERR` reader - desc RXERR"]
pub type RxerrR = crate::BitReader;
#[doc = "Field `RXERR` writer - desc RXERR"]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXERR"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXERR"]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXERR"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TxerrW<'_, ErSpec> {
        TxerrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXERR"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RxerrW<'_, ErSpec> {
        RxerrW::new(self, 1)
    }
}
#[doc = "desc ER\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErSpec;
impl crate::RegisterSpec for ErSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`er::R`](R) reader structure"]
impl crate::Readable for ErSpec {}
#[doc = "`write(|w| ..)` method takes [`er::W`](W) writer structure"]
impl crate::Writable for ErSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ER to value 0"]
impl crate::Resettable for ErSpec {}
