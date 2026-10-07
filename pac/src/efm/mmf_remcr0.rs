#[doc = "Register `MMF_REMCR0` reader"]
pub type R = crate::R<MmfRemcr0Spec>;
#[doc = "Register `MMF_REMCR0` writer"]
pub type W = crate::W<MmfRemcr0Spec>;
#[doc = "Field `RMSIZE` reader - desc RMSIZE"]
pub type RmsizeR = crate::FieldReader;
#[doc = "Field `RMSIZE` writer - desc RMSIZE"]
pub type RmsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RMTADDR` reader - desc RMTADDR"]
pub type RmtaddrR = crate::FieldReader<u32>;
#[doc = "Field `RMTADDR` writer - desc RMTADDR"]
pub type RmtaddrW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - desc RMSIZE"]
    #[inline(always)]
    pub fn rmsize(&self) -> RmsizeR {
        RmsizeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 12:28 - desc RMTADDR"]
    #[inline(always)]
    pub fn rmtaddr(&self) -> RmtaddrR {
        RmtaddrR::new((self.bits >> 12) & 0x0001_ffff)
    }
    #[doc = "Bit 31 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - desc RMSIZE"]
    #[inline(always)]
    pub fn rmsize(&mut self) -> RmsizeW<'_, MmfRemcr0Spec> {
        RmsizeW::new(self, 0)
    }
    #[doc = "Bits 12:28 - desc RMTADDR"]
    #[inline(always)]
    pub fn rmtaddr(&mut self) -> RmtaddrW<'_, MmfRemcr0Spec> {
        RmtaddrW::new(self, 12)
    }
    #[doc = "Bit 31 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, MmfRemcr0Spec> {
        EnW::new(self, 31)
    }
}
#[doc = "desc MMF_REMCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`mmf_remcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmf_remcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmfRemcr0Spec;
impl crate::RegisterSpec for MmfRemcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmf_remcr0::R`](R) reader structure"]
impl crate::Readable for MmfRemcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`mmf_remcr0::W`](W) writer structure"]
impl crate::Writable for MmfRemcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMF_REMCR0 to value 0"]
impl crate::Resettable for MmfRemcr0Spec {}
