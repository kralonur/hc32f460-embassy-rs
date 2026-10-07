#[doc = "Register `SYNCCR` reader"]
pub type R = crate::R<SynccrSpec>;
#[doc = "Register `SYNCCR` writer"]
pub type W = crate::W<SynccrSpec>;
#[doc = "Field `SYNCEN` reader - desc SYNCEN"]
pub type SyncenR = crate::BitReader;
#[doc = "Field `SYNCEN` writer - desc SYNCEN"]
pub type SyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCMD` reader - desc SYNCMD"]
pub type SyncmdR = crate::FieldReader;
#[doc = "Field `SYNCMD` writer - desc SYNCMD"]
pub type SyncmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYNCDLY` reader - desc SYNCDLY"]
pub type SyncdlyR = crate::FieldReader;
#[doc = "Field `SYNCDLY` writer - desc SYNCDLY"]
pub type SyncdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - desc SYNCEN"]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc SYNCMD"]
    #[inline(always)]
    pub fn syncmd(&self) -> SyncmdR {
        SyncmdR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - desc SYNCDLY"]
    #[inline(always)]
    pub fn syncdly(&self) -> SyncdlyR {
        SyncdlyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc SYNCEN"]
    #[inline(always)]
    pub fn syncen(&mut self) -> SyncenW<'_, SynccrSpec> {
        SyncenW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc SYNCMD"]
    #[inline(always)]
    pub fn syncmd(&mut self) -> SyncmdW<'_, SynccrSpec> {
        SyncmdW::new(self, 4)
    }
    #[doc = "Bits 8:15 - desc SYNCDLY"]
    #[inline(always)]
    pub fn syncdly(&mut self) -> SyncdlyW<'_, SynccrSpec> {
        SyncdlyW::new(self, 8)
    }
}
#[doc = "desc SYNCCR\n\nYou can [`read`](crate::Reg::read) this register and get [`synccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SynccrSpec;
impl crate::RegisterSpec for SynccrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`synccr::R`](R) reader structure"]
impl crate::Readable for SynccrSpec {}
#[doc = "`write(|w| ..)` method takes [`synccr::W`](W) writer structure"]
impl crate::Writable for SynccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCCR to value 0x0c00"]
impl crate::Resettable for SynccrSpec {
    const RESET_VALUE: u16 = 0x0c00;
}
