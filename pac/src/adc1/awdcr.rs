#[doc = "Register `AWDCR` reader"]
pub type R = crate::R<AwdcrSpec>;
#[doc = "Register `AWDCR` writer"]
pub type W = crate::W<AwdcrSpec>;
#[doc = "Field `AWDEN` reader - desc AWDEN"]
pub type AwdenR = crate::BitReader;
#[doc = "Field `AWDEN` writer - desc AWDEN"]
pub type AwdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDMD` reader - desc AWDMD"]
pub type AwdmdR = crate::BitReader;
#[doc = "Field `AWDMD` writer - desc AWDMD"]
pub type AwdmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDSS` reader - desc AWDSS"]
pub type AwdssR = crate::FieldReader;
#[doc = "Field `AWDSS` writer - desc AWDSS"]
pub type AwdssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AWDIEN` reader - desc AWDIEN"]
pub type AwdienR = crate::BitReader;
#[doc = "Field `AWDIEN` writer - desc AWDIEN"]
pub type AwdienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc AWDEN"]
    #[inline(always)]
    pub fn awden(&self) -> AwdenR {
        AwdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - desc AWDMD"]
    #[inline(always)]
    pub fn awdmd(&self) -> AwdmdR {
        AwdmdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc AWDSS"]
    #[inline(always)]
    pub fn awdss(&self) -> AwdssR {
        AwdssR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc AWDIEN"]
    #[inline(always)]
    pub fn awdien(&self) -> AwdienR {
        AwdienR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AWDEN"]
    #[inline(always)]
    pub fn awden(&mut self) -> AwdenW<'_, AwdcrSpec> {
        AwdenW::new(self, 0)
    }
    #[doc = "Bit 4 - desc AWDMD"]
    #[inline(always)]
    pub fn awdmd(&mut self) -> AwdmdW<'_, AwdcrSpec> {
        AwdmdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc AWDSS"]
    #[inline(always)]
    pub fn awdss(&mut self) -> AwdssW<'_, AwdcrSpec> {
        AwdssW::new(self, 6)
    }
    #[doc = "Bit 8 - desc AWDIEN"]
    #[inline(always)]
    pub fn awdien(&mut self) -> AwdienW<'_, AwdcrSpec> {
        AwdienW::new(self, 8)
    }
}
#[doc = "desc AWDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdcrSpec;
impl crate::RegisterSpec for AwdcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`awdcr::R`](R) reader structure"]
impl crate::Readable for AwdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`awdcr::W`](W) writer structure"]
impl crate::Writable for AwdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDCR to value 0"]
impl crate::Resettable for AwdcrSpec {}
